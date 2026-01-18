//! Data Component Definitions
//!
//! This module provides component definitions for data modeling:
//! - Entity: Core data model/table definition
//! - Collection: Array/list of entities
//! - Query: Database query builder

use crate::definition::{
    ComponentDefinition, ConfigOption, ConfigType, FieldDefinition, PortDefinition,
};
use imortal_core::{ComponentCategory, DataType, Validation};

/// Create the Entity component definition
///
/// An Entity represents a data model - similar to a database table or struct.
/// Users can add fields, set constraints, and define relationships.
pub fn entity_component() -> ComponentDefinition {
    ComponentDefinition::new("data.entity", "Entity", ComponentCategory::Data)
        .with_description("Define a data model with fields and relationships")
        .with_icon("📊")
        .with_field(
            FieldDefinition::uuid("id")
                .with_label("ID")
                .with_description("Primary key identifier")
                .required()
                .read_only(),
        )
        .with_field(
            FieldDefinition::datetime("created_at")
                .with_label("Created At")
                .with_description("Timestamp when the record was created")
                .read_only(),
        )
        .with_field(
            FieldDefinition::datetime("updated_at")
                .with_label("Updated At")
                .with_description("Timestamp when the record was last updated")
                .read_only(),
        )
        .with_output(
            PortDefinition::data_out("entity", "Entity", DataType::Entity("Self".to_string()))
                .with_description("The entity type for creating relationships"),
        )
        .with_output(
            PortDefinition::data_out("list", "List", DataType::Array(Box::new(DataType::Entity("Self".to_string()))))
                .with_description("A list/array of this entity type"),
        )
        .with_input(
            PortDefinition::trigger_in("create", "On Create")
                .with_description("Triggered when a new record is created"),
        )
        .with_input(
            PortDefinition::trigger_in("update", "On Update")
                .with_description("Triggered when a record is updated"),
        )
        .with_input(
            PortDefinition::trigger_in("delete", "On Delete")
                .with_description("Triggered when a record is deleted"),
        )
        .with_config(
            ConfigOption::string("table_name", "Table Name")
                .with_description("Database table name (defaults to snake_case of entity name)")
                .with_default(""),
        )
        .with_config(
            ConfigOption::boolean("timestamps", "Auto Timestamps")
                .with_description("Automatically manage created_at and updated_at fields")
                .with_default(true),
        )
        .with_config(
            ConfigOption::boolean("soft_delete", "Soft Delete")
                .with_description("Use soft delete instead of hard delete")
                .with_default(false),
        )
        .with_config(
            ConfigOption::select("id_type", "ID Type")
                .with_option("uuid", "UUID")
                .with_option("auto_increment", "Auto Increment")
                .with_option("custom", "Custom")
                .with_default("uuid")
                .with_description("Type of primary key to use"),
        )
        .allow_custom_fields()
        .with_default_size(220.0, 200.0)
        .with_generator("data::entity")
        .with_tag("model")
        .with_tag("table")
        .with_tag("struct")
}

/// Create the Collection component definition
///
/// A Collection represents a list/array of entities with filtering,
/// sorting, and pagination capabilities.
pub fn collection_component() -> ComponentDefinition {
    ComponentDefinition::new("data.collection", "Collection", ComponentCategory::Data)
        .with_description("A queryable collection of entities with filtering and pagination")
        .with_icon("📚")
        .with_input(
            PortDefinition::data_in("entity_type", "Entity Type", DataType::Entity("Any".to_string()))
                .with_description("The type of entities in this collection")
                .required(),
        )
        .with_input(
            PortDefinition::data_in("filter", "Filter", DataType::Json)
                .with_description("Filter criteria for the collection"),
        )
        .with_input(
            PortDefinition::trigger_in("refresh", "Refresh")
                .with_description("Refresh the collection data"),
        )
        .with_output(
            PortDefinition::data_out("items", "Items", DataType::Array(Box::new(DataType::Any)))
                .with_description("The filtered and paginated items"),
        )
        .with_output(
            PortDefinition::data_out("count", "Count", DataType::Int64)
                .with_description("Total count of items matching the filter"),
        )
        .with_output(
            PortDefinition::data_out("page_info", "Page Info", DataType::Json)
                .with_description("Pagination information"),
        )
        .with_output(
            PortDefinition::trigger_out("on_load", "On Load")
                .with_description("Triggered when collection is loaded"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when an error occurs"),
        )
        .with_config(
            ConfigOption::integer("page_size", "Page Size")
                .with_description("Number of items per page")
                .with_default(imortal_core::ConfigValue::Int(25))
                .with_min(1.0)
                .with_max(1000.0),
        )
        .with_config(
            ConfigOption::string("default_sort", "Default Sort")
                .with_description("Default sort field (prefix with - for descending)")
                .with_default("-created_at"),
        )
        .with_config(
            ConfigOption::boolean("auto_load", "Auto Load")
                .with_description("Automatically load collection on initialization")
                .with_default(true),
        )
        .with_config(
            ConfigOption::boolean("cache", "Enable Cache")
                .with_description("Cache collection results")
                .with_default(false),
        )
        .with_config(
            ConfigOption::integer("cache_ttl", "Cache TTL")
                .with_description("Cache time-to-live in seconds")
                .with_default(imortal_core::ConfigValue::Int(300))
                .in_group("Cache"),
        )
        .with_default_size(200.0, 180.0)
        .with_generator("data::collection")
        .with_tag("list")
        .with_tag("query")
        .with_tag("filter")
}

/// Create the Query component definition
///
/// A Query component for building and executing database queries
/// with support for complex conditions, joins, and aggregations.
pub fn query_component() -> ComponentDefinition {
    ComponentDefinition::new("data.query", "Query", ComponentCategory::Data)
        .with_description("Build and execute database queries with conditions and joins")
        .with_icon("🔍")
        .with_input(
            PortDefinition::data_in("source", "Source", DataType::Entity("Any".to_string()))
                .with_description("The entity/table to query from")
                .required(),
        )
        .with_input(
            PortDefinition::data_in("params", "Parameters", DataType::Json)
                .with_description("Query parameters for prepared statements"),
        )
        .with_input(
            PortDefinition::trigger_in("execute", "Execute")
                .with_description("Execute the query"),
        )
        .with_output(
            PortDefinition::data_out("results", "Results", DataType::Array(Box::new(DataType::Any)))
                .with_description("Query results"),
        )
        .with_output(
            PortDefinition::data_out("first", "First", DataType::Any)
                .with_description("First result or null"),
        )
        .with_output(
            PortDefinition::data_out("count", "Count", DataType::Int64)
                .with_description("Number of results"),
        )
        .with_output(
            PortDefinition::trigger_out("on_success", "On Success")
                .with_description("Triggered when query succeeds"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when query fails"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message if query fails"),
        )
        .with_field(
            FieldDefinition::text("where_clause")
                .with_label("Where")
                .with_placeholder("field = $1 AND other_field > $2")
                .with_description("WHERE clause conditions"),
        )
        .with_field(
            FieldDefinition::string("order_by")
                .with_label("Order By")
                .with_placeholder("created_at DESC")
                .with_description("ORDER BY clause"),
        )
        .with_field(
            FieldDefinition::int("limit")
                .with_label("Limit")
                .with_default(imortal_core::ConfigValue::Int(100))
                .with_description("Maximum number of results"),
        )
        .with_field(
            FieldDefinition::int("offset")
                .with_label("Offset")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_description("Number of results to skip"),
        )
        .with_config(
            ConfigOption::select("query_type", "Query Type")
                .with_option("select", "SELECT")
                .with_option("insert", "INSERT")
                .with_option("update", "UPDATE")
                .with_option("delete", "DELETE")
                .with_default("select")
                .with_description("Type of SQL query"),
        )
        .with_config(
            ConfigOption::string("select_fields", "Select Fields")
                .with_description("Fields to select (comma-separated, * for all)")
                .with_default("*"),
        )
        .with_config(
            ConfigOption::boolean("distinct", "Distinct")
                .with_description("Return only distinct results")
                .with_default(false),
        )
        .with_config(
            ConfigOption::string("joins", "Joins")
                .with_description("JOIN clauses")
                .advanced(),
        )
        .with_config(
            ConfigOption::string("group_by", "Group By")
                .with_description("GROUP BY clause")
                .advanced(),
        )
        .with_config(
            ConfigOption::string("having", "Having")
                .with_description("HAVING clause")
                .advanced(),
        )
        .with_default_size(220.0, 220.0)
        .with_generator("data::query")
        .with_tag("sql")
        .with_tag("database")
        .with_tag("filter")
}

/// Create a Relationship component definition
///
/// Defines relationships between entities (one-to-one, one-to-many, many-to-many)
pub fn relationship_component() -> ComponentDefinition {
    ComponentDefinition::new("data.relationship", "Relationship", ComponentCategory::Data)
        .with_description("Define relationships between entities")
        .with_icon("🔗")
        .with_input(
            PortDefinition::data_in("from", "From Entity", DataType::Entity("Any".to_string()))
                .with_description("Source entity of the relationship")
                .required(),
        )
        .with_input(
            PortDefinition::data_in("to", "To Entity", DataType::Entity("Any".to_string()))
                .with_description("Target entity of the relationship")
                .required(),
        )
        .with_output(
            PortDefinition::data_out("relation", "Relation", DataType::Any)
                .with_description("The relationship definition"),
        )
        .with_config(
            ConfigOption::select("type", "Relationship Type")
                .with_option("one_to_one", "One to One")
                .with_option("one_to_many", "One to Many")
                .with_option("many_to_one", "Many to One")
                .with_option("many_to_many", "Many to Many")
                .with_default("one_to_many")
                .required()
                .with_description("Type of relationship between entities"),
        )
        .with_config(
            ConfigOption::string("foreign_key", "Foreign Key")
                .with_description("Foreign key field name (auto-generated if empty)"),
        )
        .with_config(
            ConfigOption::string("through_table", "Through Table")
                .with_description("Junction table name for many-to-many relationships")
                .advanced(),
        )
        .with_config(
            ConfigOption::select("on_delete", "On Delete")
                .with_option("cascade", "Cascade")
                .with_option("set_null", "Set Null")
                .with_option("restrict", "Restrict")
                .with_option("no_action", "No Action")
                .with_default("cascade")
                .with_description("Action when parent record is deleted"),
        )
        .with_config(
            ConfigOption::select("on_update", "On Update")
                .with_option("cascade", "Cascade")
                .with_option("set_null", "Set Null")
                .with_option("restrict", "Restrict")
                .with_option("no_action", "No Action")
                .with_default("cascade")
                .with_description("Action when parent record is updated"),
        )
        .with_config(
            ConfigOption::boolean("eager_load", "Eager Load")
                .with_description("Load related entities automatically")
                .with_default(false),
        )
        .with_default_size(180.0, 120.0)
        .with_generator("data::relationship")
        .with_tag("relation")
        .with_tag("foreign_key")
        .with_tag("join")
}

/// Create a Computed Field component definition
///
/// A virtual/computed field that derives its value from other fields
pub fn computed_field_component() -> ComponentDefinition {
    ComponentDefinition::new("data.computed", "Computed Field", ComponentCategory::Data)
        .with_description("A virtual field computed from other fields")
        .with_icon("🧮")
        .with_input(
            PortDefinition::data_in("inputs", "Inputs", DataType::Any)
                .with_description("Input values for computation")
                .multiple(),
        )
        .with_output(
            PortDefinition::data_out("value", "Value", DataType::Any)
                .with_description("The computed value"),
        )
        .with_field(
            FieldDefinition::string("name")
                .with_label("Field Name")
                .required()
                .with_placeholder("full_name")
                .with_description("Name of the computed field"),
        )
        .with_field(
            FieldDefinition::text("expression")
                .with_label("Expression")
                .required()
                .with_placeholder("first_name || ' ' || last_name")
                .with_description("Expression to compute the value"),
        )
        .with_config(
            ConfigOption::select("output_type", "Output Type")
                .with_option("string", "String")
                .with_option("int", "Integer")
                .with_option("float", "Float")
                .with_option("bool", "Boolean")
                .with_option("datetime", "DateTime")
                .with_default("string")
                .with_description("Data type of the computed value"),
        )
        .with_config(
            ConfigOption::boolean("stored", "Stored")
                .with_description("Store the computed value in the database")
                .with_default(false),
        )
        .with_default_size(180.0, 140.0)
        .with_generator("data::computed")
        .with_tag("virtual")
        .with_tag("derived")
        .with_tag("expression")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_component() {
        let def = entity_component();
        assert_eq!(def.id, "data.entity");
        assert_eq!(def.category, ComponentCategory::Data);
        assert!(def.allow_custom_fields);
        assert!(def.fields.iter().any(|f| f.name == "id"));
    }

    #[test]
    fn test_collection_component() {
        let def = collection_component();
        assert_eq!(def.id, "data.collection");
        assert!(def.ports.inputs.iter().any(|p| p.id == "entity_type"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "items"));
    }

    #[test]
    fn test_query_component() {
        let def = query_component();
        assert_eq!(def.id, "data.query");
        assert!(def.config.iter().any(|c| c.id == "query_type"));
        assert!(def.fields.iter().any(|f| f.name == "where_clause"));
    }

    #[test]
    fn test_entity_instantiation() {
        let def = entity_component();
        let node = def.instantiate("User");

        assert_eq!(node.name, "User");
        assert_eq!(node.component_type, "data.entity");
        assert!(node.fields.iter().any(|f| f.name == "id"));
    }

    #[test]
    fn test_relationship_component() {
        let def = relationship_component();
        assert_eq!(def.id, "data.relationship");
        assert!(def.config.iter().any(|c| c.id == "type"));
        assert!(def.config.iter().any(|c| c.id == "on_delete"));
    }
}
