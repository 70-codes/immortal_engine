//! Database migration generation for Immortal Engine
//!
//! This module provides utilities for generating SQL database migrations
//! from entity nodes in the project graph.

use std::collections::HashMap;
use imortal_ir::{Node, Field, ProjectGraph};
use imortal_core::{DataType, EngineResult, EngineError};


/// Supported database backends for migration generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DatabaseBackend {
    #[default]
    Postgres,
    Sqlite,
    Mysql,
}

impl DatabaseBackend {
    /// Get the SQL type for a DataType in this backend
    pub fn sql_type(&self, data_type: &DataType) -> String {
        match self {
            DatabaseBackend::Postgres => postgres_type(data_type),
            DatabaseBackend::Sqlite => sqlite_type(data_type),
            DatabaseBackend::Mysql => mysql_type(data_type),
        }
    }

    /// Get the auto-increment syntax for this backend
    pub fn auto_increment(&self) -> &'static str {
        match self {
            DatabaseBackend::Postgres => "SERIAL",
            DatabaseBackend::Sqlite => "INTEGER PRIMARY KEY AUTOINCREMENT",
            DatabaseBackend::Mysql => "AUTO_INCREMENT",
        }
    }

    /// Get the current timestamp default for this backend
    pub fn current_timestamp(&self) -> &'static str {
        match self {
            DatabaseBackend::Postgres => "NOW()",
            DatabaseBackend::Sqlite => "CURRENT_TIMESTAMP",
            DatabaseBackend::Mysql => "CURRENT_TIMESTAMP",
        }
    }
}

/// A generated migration
#[derive(Debug, Clone)]
pub struct Migration {
    /// Migration name/version
    pub name: String,
    /// SQL to apply the migration
    pub up: String,
    /// SQL to rollback the migration
    pub down: String,
    /// Timestamp or version number
    pub version: String,
}

impl Migration {
    /// Create a new migration
    pub fn new(name: impl Into<String>, up: impl Into<String>, down: impl Into<String>) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        Self {
            name: name.into(),
            up: up.into(),
            down: down.into(),
            version: timestamp,
        }
    }

    /// Create with a specific version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Get the full migration filename
    pub fn filename(&self) -> String {
        format!("{}_{}.sql", self.version, to_snake_case(&self.name))
    }

    /// Generate the full migration file content
    pub fn to_sql(&self) -> String {
        format!(
            "-- Migration: {}\n-- Version: {}\n\n-- Up\n{}\n\n-- Down\n{}\n",
            self.name,
            self.version,
            self.up,
            self.down
        )
    }
}

/// Migration generator configuration
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Target database backend
    pub backend: DatabaseBackend,
    /// Schema name (if applicable)
    pub schema: Option<String>,
    /// Whether to generate indexes
    pub generate_indexes: bool,
    /// Whether to add created_at/updated_at timestamps
    pub add_timestamps: bool,
    /// Whether to use UUID for primary keys
    pub uuid_primary_keys: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            backend: DatabaseBackend::Postgres,
            schema: None,
            generate_indexes: true,
            add_timestamps: true,
            uuid_primary_keys: true,
        }
    }
}

impl MigrationConfig {
    /// Create a new config for Postgres
    pub fn postgres() -> Self {
        Self::default()
    }

    /// Create a new config for SQLite
    pub fn sqlite() -> Self {
        Self {
            backend: DatabaseBackend::Sqlite,
            uuid_primary_keys: false, // SQLite handles UUIDs as TEXT
            ..Default::default()
        }
    }

    /// Create a new config for MySQL
    pub fn mysql() -> Self {
        Self {
            backend: DatabaseBackend::Mysql,
            ..Default::default()
        }
    }

    /// Set the schema name
    pub fn with_schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = Some(schema.into());
        self
    }
}

/// Generate migrations from a project graph
pub struct MigrationGenerator {
    config: MigrationConfig,
}

impl MigrationGenerator {
    /// Create a new migration generator
    pub fn new(config: MigrationConfig) -> Self {
        Self { config }
    }

    /// Generate all migrations for a project
    pub fn generate(&self, graph: &ProjectGraph) -> EngineResult<Vec<Migration>> {
        let mut migrations = Vec::new();

        // Find all entity nodes
        let entities: Vec<&Node> = graph.nodes()
            .filter(|n| n.component_type == "data.entity")
            .collect();

        if entities.is_empty() {
            return Ok(migrations);
        }

        // Generate initial schema migration
        let schema_migration = self.generate_schema_migration(&entities)?;
        migrations.push(schema_migration);

        // Generate index migrations if enabled
        if self.config.generate_indexes {
            if let Some(index_migration) = self.generate_index_migration(&entities)? {
                migrations.push(index_migration);
            }
        }

        // Generate foreign key migrations
        let fk_migrations = self.generate_foreign_key_migrations(&entities, graph)?;
        migrations.extend(fk_migrations);

        Ok(migrations)
    }

    /// Generate a migration for a single entity
    pub fn generate_for_entity(&self, node: &Node) -> EngineResult<Migration> {
        if node.component_type != "data.entity" {
            return Err(EngineError::InvalidComponentConfig(
                format!("Node '{}' is not an entity", node.name)
            ));
        }

        let up_sql = self.generate_create_table(node)?;
        let down_sql = self.generate_drop_table(&node.name);

        Ok(Migration::new(
            format!("create_{}", to_snake_case(&node.name)),
            up_sql,
            down_sql,
        ))
    }

    /// Generate the schema migration (all tables)
    fn generate_schema_migration(&self, entities: &[&Node]) -> EngineResult<Migration> {
        let mut up_statements = Vec::new();
        let mut down_statements = Vec::new();

        for entity in entities {
            up_statements.push(self.generate_create_table(entity)?);
            down_statements.push(self.generate_drop_table(&entity.name));
        }

        // Reverse down statements so tables are dropped in correct order
        down_statements.reverse();

        Ok(Migration::new(
            "initial_schema",
            up_statements.join("\n\n"),
            down_statements.join("\n\n"),
        ))
    }

    /// Generate CREATE TABLE statement for an entity
    fn generate_create_table(&self, node: &Node) -> EngineResult<String> {
        let table_name = self.table_name(&node.name);
        let mut columns = Vec::new();
        let mut constraints = Vec::new();

        // Process fields
        for field in &node.fields {
            let column_def = self.generate_column_definition(field)?;
            columns.push(column_def);

            // Collect constraints
            if let Some(constraint) = self.generate_column_constraint(field, &table_name) {
                constraints.push(constraint);
            }
        }

        // Add timestamps if configured
        if self.config.add_timestamps {
            if !node.fields.iter().any(|f| f.name == "created_at") {
                columns.push(format!(
                    "    created_at TIMESTAMP WITH TIME ZONE DEFAULT {} NOT NULL",
                    self.config.backend.current_timestamp()
                ));
            }
            if !node.fields.iter().any(|f| f.name == "updated_at") {
                columns.push(format!(
                    "    updated_at TIMESTAMP WITH TIME ZONE DEFAULT {} NOT NULL",
                    self.config.backend.current_timestamp()
                ));
            }
        }

        let mut sql = format!(
            "CREATE TABLE {} (\n{}\n)",
            table_name,
            columns.join(",\n")
        );

        // Add table-level constraints
        if !constraints.is_empty() {
            sql.push_str(";\n\n");
            sql.push_str(&constraints.join(";\n"));
        }

        sql.push(';');

        Ok(sql)
    }

    /// Generate a column definition
    fn generate_column_definition(&self, field: &Field) -> EngineResult<String> {
        let column_name = to_snake_case(&field.name);
        let sql_type = self.config.backend.sql_type(&field.data_type);

        let mut parts = vec![format!("    {}", column_name), sql_type];

        // Handle constraints
        for constraint in &field.constraints {
            match constraint {
                imortal_ir::field::FieldConstraint::PrimaryKey => {
                    parts.push("PRIMARY KEY".to_string());
                }
                imortal_ir::field::FieldConstraint::Unique => {
                    parts.push("UNIQUE".to_string());
                }
                imortal_ir::field::FieldConstraint::AutoIncrement => {
                    // Handled by sql_type for some backends
                }
                imortal_ir::field::FieldConstraint::DefaultExpression(expr) => {
                    parts.push(format!("DEFAULT {}", expr));
                }
                _ => {}
            }
        }

        // NOT NULL for required fields
        if field.required && !field.is_primary_key() {
            parts.push("NOT NULL".to_string());
        }

        // Default value
        if let Some(default) = &field.default_value {
            if !parts.iter().any(|p| p.starts_with("DEFAULT")) {
                let default_str = config_value_to_sql(default);
                parts.push(format!("DEFAULT {}", default_str));
            }
        }

        Ok(parts.join(" "))
    }

    /// Generate column-level constraints (foreign keys, etc.)
    fn generate_column_constraint(&self, field: &Field, table_name: &str) -> Option<String> {
        for constraint in &field.constraints {
            if let imortal_ir::field::FieldConstraint::ForeignKey {
                entity,
                field: ref_field,
                on_delete,
                on_update,
            } = constraint {
                let fk_name = format!(
                    "fk_{}_{}_{}",
                    to_snake_case(table_name),
                    to_snake_case(&field.name),
                    to_snake_case(entity)
                );
                let ref_table = self.table_name(entity);

                return Some(format!(
                    "ALTER TABLE {} ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE {} ON UPDATE {}",
                    table_name,
                    fk_name,
                    to_snake_case(&field.name),
                    ref_table,
                    to_snake_case(ref_field),
                    on_delete.to_sql(),
                    on_update.to_sql()
                ));
            }
        }
        None
    }

    /// Generate DROP TABLE statement
    fn generate_drop_table(&self, name: &str) -> String {
        format!("DROP TABLE IF EXISTS {} CASCADE;", self.table_name(name))
    }

    /// Generate index migration
    fn generate_index_migration(&self, entities: &[&Node]) -> EngineResult<Option<Migration>> {
        let mut up_statements = Vec::new();
        let mut down_statements = Vec::new();

        for entity in entities {
            let table_name = self.table_name(&entity.name);

            for field in &entity.fields {
                // Create indexes for indexed fields
                if field.constraints.iter().any(|c| matches!(c, imortal_ir::field::FieldConstraint::Indexed)) {
                    let index_name = format!("idx_{}_{}", to_snake_case(&entity.name), to_snake_case(&field.name));
                    let column_name = to_snake_case(&field.name);

                    up_statements.push(format!(
                        "CREATE INDEX {} ON {} ({});",
                        index_name, table_name, column_name
                    ));
                    down_statements.push(format!("DROP INDEX IF EXISTS {};", index_name));
                }

                // Create indexes for foreign key fields
                if field.is_foreign_key() {
                    let index_name = format!("idx_{}_{}", to_snake_case(&entity.name), to_snake_case(&field.name));
                    let column_name = to_snake_case(&field.name);

                    // Avoid duplicate index
                    let create_stmt = format!(
                        "CREATE INDEX IF NOT EXISTS {} ON {} ({});",
                        index_name, table_name, column_name
                    );
                    if !up_statements.contains(&create_stmt) {
                        up_statements.push(create_stmt);
                        down_statements.push(format!("DROP INDEX IF EXISTS {};", index_name));
                    }
                }
            }
        }

        if up_statements.is_empty() {
            return Ok(None);
        }

        Ok(Some(Migration::new(
            "create_indexes",
            up_statements.join("\n"),
            down_statements.join("\n"),
        )))
    }

    /// Generate foreign key migrations
    fn generate_foreign_key_migrations(&self, entities: &[&Node], _graph: &ProjectGraph) -> EngineResult<Vec<Migration>> {
        let mut migrations = Vec::new();
        let mut fk_statements = Vec::new();
        let mut drop_statements = Vec::new();

        for entity in entities {
            let table_name = self.table_name(&entity.name);

            for field in &entity.fields {
                if let Some(fk_sql) = self.generate_column_constraint(field, &table_name) {
                    fk_statements.push(format!("{};", fk_sql));

                    let fk_name = format!(
                        "fk_{}_{}_{}",
                        to_snake_case(&table_name),
                        to_snake_case(&field.name),
                        "ref"
                    );
                    drop_statements.push(format!(
                        "ALTER TABLE {} DROP CONSTRAINT IF EXISTS {};",
                        table_name, fk_name
                    ));
                }
            }
        }

        if !fk_statements.is_empty() {
            migrations.push(Migration::new(
                "add_foreign_keys",
                fk_statements.join("\n"),
                drop_statements.join("\n"),
            ));
        }

        Ok(migrations)
    }

    /// Get the full table name (with schema if applicable)
    fn table_name(&self, name: &str) -> String {
        let snake_name = to_snake_case(name);
        if let Some(ref schema) = self.config.schema {
            format!("{}.{}", schema, snake_name)
        } else {
            snake_name
        }
    }
}

/// Get SQL type for Postgres
fn postgres_type(data_type: &DataType) -> String {
    match data_type {
        DataType::String => "VARCHAR(255)".to_string(),
        DataType::Text => "TEXT".to_string(),
        DataType::Int32 => "INTEGER".to_string(),
        DataType::Int64 => "BIGINT".to_string(),
        DataType::Float32 => "REAL".to_string(),
        DataType::Float64 => "DOUBLE PRECISION".to_string(),
        DataType::Bool => "BOOLEAN".to_string(),
        DataType::Uuid => "UUID".to_string(),
        DataType::DateTime => "TIMESTAMP WITH TIME ZONE".to_string(),
        DataType::Date => "DATE".to_string(),
        DataType::Time => "TIME".to_string(),
        DataType::Bytes => "BYTEA".to_string(),
        DataType::Json => "JSONB".to_string(),
        DataType::Optional(inner) => postgres_type(inner),
        DataType::Array(inner) => format!("{}[]", postgres_type(inner)),
        DataType::Reference(_) | DataType::Entity(_) => "UUID".to_string(),
        DataType::Any => "JSONB".to_string(),
        _ => "TEXT".to_string(),
    }
}

/// Get SQL type for SQLite
fn sqlite_type(data_type: &DataType) -> String {
    match data_type {
        DataType::String | DataType::Text => "TEXT".to_string(),
        DataType::Int32 | DataType::Int64 => "INTEGER".to_string(),
        DataType::Float32 | DataType::Float64 => "REAL".to_string(),
        DataType::Bool => "INTEGER".to_string(), // SQLite uses 0/1
        DataType::Uuid => "TEXT".to_string(),    // SQLite stores UUIDs as text
        DataType::DateTime | DataType::Date | DataType::Time => "TEXT".to_string(),
        DataType::Bytes => "BLOB".to_string(),
        DataType::Json => "TEXT".to_string(),
        DataType::Optional(inner) => sqlite_type(inner),
        DataType::Array(_) => "TEXT".to_string(), // Store as JSON
        DataType::Reference(_) | DataType::Entity(_) => "TEXT".to_string(),
        DataType::Any => "TEXT".to_string(),
        _ => "TEXT".to_string(),
    }
}

/// Get SQL type for MySQL
fn mysql_type(data_type: &DataType) -> String {
    match data_type {
        DataType::String => "VARCHAR(255)".to_string(),
        DataType::Text => "TEXT".to_string(),
        DataType::Int32 => "INT".to_string(),
        DataType::Int64 => "BIGINT".to_string(),
        DataType::Float32 => "FLOAT".to_string(),
        DataType::Float64 => "DOUBLE".to_string(),
        DataType::Bool => "TINYINT(1)".to_string(),
        DataType::Uuid => "CHAR(36)".to_string(),
        DataType::DateTime => "DATETIME".to_string(),
        DataType::Date => "DATE".to_string(),
        DataType::Time => "TIME".to_string(),
        DataType::Bytes => "BLOB".to_string(),
        DataType::Json => "JSON".to_string(),
        DataType::Optional(inner) => mysql_type(inner),
        DataType::Array(_) => "JSON".to_string(),
        DataType::Reference(_) | DataType::Entity(_) => "CHAR(36)".to_string(),
        DataType::Any => "JSON".to_string(),
        _ => "TEXT".to_string(),
    }
}

/// Convert ConfigValue to SQL literal
fn config_value_to_sql(value: &imortal_core::ConfigValue) -> String {
    match value {
        imortal_core::ConfigValue::Null => "NULL".to_string(),
        imortal_core::ConfigValue::Bool(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
        imortal_core::ConfigValue::Int(i) => i.to_string(),
        imortal_core::ConfigValue::Float(f) => f.to_string(),
        imortal_core::ConfigValue::String(s) => format!("'{}'", s.replace('\'', "''")),
        imortal_core::ConfigValue::Array(_) => "NULL".to_string(),
        imortal_core::ConfigValue::Object(_) => "NULL".to_string(),
    }
}

/// Convert string to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_is_upper = true;
        } else if c == '-' || c == ' ' {
            result.push('_');
            prev_is_upper = false;
        } else {
            result.push(c);
            prev_is_upper = false;
        }
    }

    result
}

/// Utility to generate a complete migration file
pub fn generate_migration_file(migration: &Migration) -> String {
    migration.to_sql()
}

/// Generate all migration files for a project
pub fn generate_all_migrations(
    graph: &ProjectGraph,
    backend: DatabaseBackend,
) -> EngineResult<HashMap<String, String>> {
    let config = MigrationConfig {
        backend,
        ..Default::default()
    };

    let generator = MigrationGenerator::new(config);
    let migrations = generator.generate(graph)?;

    let mut files = HashMap::new();
    for migration in migrations {
        files.insert(migration.filename(), migration.to_sql());
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use imortal_ir::{ProjectMeta, Field};

    fn create_test_entity() -> Node {
        Node::new_entity("User")
            .with_field(Field::uuid("id").primary_key())
            .with_field(Field::string("email").required().unique())
            .with_field(Field::string("name").required())
            .with_field(Field::bool("active").with_default(true))
    }

    #[test]
    fn test_migration_generation() {
        let entity = create_test_entity();
        let config = MigrationConfig::postgres();
        let generator = MigrationGenerator::new(config);

        let migration = generator.generate_for_entity(&entity).unwrap();

        assert!(migration.up.contains("CREATE TABLE"));
        assert!(migration.up.contains("user"));
        assert!(migration.down.contains("DROP TABLE"));
    }

    #[test]
    fn test_postgres_types() {
        assert_eq!(postgres_type(&DataType::String), "VARCHAR(255)");
        assert_eq!(postgres_type(&DataType::Uuid), "UUID");
        assert_eq!(postgres_type(&DataType::Bool), "BOOLEAN");
        assert_eq!(postgres_type(&DataType::Json), "JSONB");
    }

    #[test]
    fn test_sqlite_types() {
        assert_eq!(sqlite_type(&DataType::String), "TEXT");
        assert_eq!(sqlite_type(&DataType::Uuid), "TEXT");
        assert_eq!(sqlite_type(&DataType::Bool), "INTEGER");
    }

    #[test]
    fn test_mysql_types() {
        assert_eq!(mysql_type(&DataType::String), "VARCHAR(255)");
        assert_eq!(mysql_type(&DataType::Uuid), "CHAR(36)");
        assert_eq!(mysql_type(&DataType::Bool), "TINYINT(1)");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("UserProfile"), "user_profile");
        assert_eq!(to_snake_case("user"), "user");
        assert_eq!(to_snake_case("MyAPIKey"), "my_apikey");
    }

    #[test]
    fn test_migration_filename() {
        let migration = Migration::new("create_users", "CREATE TABLE...", "DROP TABLE...")
            .with_version("20240101120000");

        assert_eq!(migration.filename(), "20240101120000_create_users.sql");
    }

    #[test]
    fn test_config_value_to_sql() {
        assert_eq!(config_value_to_sql(&imortal_core::ConfigValue::Bool(true)), "TRUE");
        assert_eq!(config_value_to_sql(&imortal_core::ConfigValue::Int(42)), "42");
        assert_eq!(config_value_to_sql(&imortal_core::ConfigValue::String("test".into())), "'test'");
    }
}
