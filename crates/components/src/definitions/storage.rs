//! Storage Component Definitions
//!
//! This module provides component definitions for data storage:
//! - Database: Database connection and configuration
//! - Cache: In-memory or distributed caching
//! - FileStorage: File/blob storage

use crate::definition::{
    ComponentDefinition, ConfigOption, ConfigType, FieldDefinition, PortDefinition,
};
use imortal_core::{ComponentCategory, DataType, Validation};

/// Create the Database component definition
///
/// Represents a database connection with support for multiple backends
/// (PostgreSQL, MySQL, SQLite, etc.)
pub fn database_component() -> ComponentDefinition {
    ComponentDefinition::new("storage.database", "Database", ComponentCategory::Storage)
        .with_description("Database connection and configuration for data persistence")
        .with_icon("💾")
        .with_tag("persistence")
        .with_tag("sql")
        .with_tag("data")
        // Input ports
        .with_input(
            PortDefinition::data_in("query", "Query", DataType::String)
                .with_description("SQL query to execute"),
        )
        .with_input(
            PortDefinition::data_in("params", "Parameters", DataType::Array(Box::new(DataType::Any)))
                .with_description("Query parameters for prepared statements"),
        )
        .with_input(
            PortDefinition::trigger_in("connect", "Connect")
                .with_description("Establish database connection"),
        )
        .with_input(
            PortDefinition::trigger_in("disconnect", "Disconnect")
                .with_description("Close database connection"),
        )
        .with_input(
            PortDefinition::trigger_in("execute", "Execute")
                .with_description("Execute a query"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("connection", "Connection", DataType::Any)
                .with_description("Database connection for other components"),
        )
        .with_output(
            PortDefinition::data_out("result", "Result", DataType::Array(Box::new(DataType::Any)))
                .with_description("Query results"),
        )
        .with_output(
            PortDefinition::data_out("affected_rows", "Affected Rows", DataType::Int64)
                .with_description("Number of rows affected by the query"),
        )
        .with_output(
            PortDefinition::trigger_out("on_connect", "On Connect")
                .with_description("Triggered when connection is established"),
        )
        .with_output(
            PortDefinition::trigger_out("on_disconnect", "On Disconnect")
                .with_description("Triggered when connection is closed"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when an error occurs"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message if operation fails"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("backend", "Database Backend")
                .with_option("postgres", "PostgreSQL")
                .with_option("mysql", "MySQL")
                .with_option("sqlite", "SQLite")
                .with_option("mssql", "Microsoft SQL Server")
                .with_option("mongodb", "MongoDB")
                .with_default("postgres")
                .required()
                .with_description("Database backend to use"),
        )
        .with_config(
            ConfigOption::string("connection_string", "Connection String")
                .with_description("Database connection string (use environment variable)")
                .with_default("${DATABASE_URL}"),
        )
        .with_config(
            ConfigOption::string("host", "Host")
                .with_description("Database host address")
                .with_default("localhost")
                .in_group("Connection"),
        )
        .with_config(
            ConfigOption::integer("port", "Port")
                .with_description("Database port")
                .with_default(imortal_core::ConfigValue::Int(5432))
                .in_group("Connection"),
        )
        .with_config(
            ConfigOption::string("database", "Database Name")
                .with_description("Name of the database to connect to")
                .with_default("")
                .in_group("Connection"),
        )
        .with_config(
            ConfigOption::string("username", "Username")
                .with_description("Database username")
                .with_default("")
                .in_group("Connection"),
        )
        .with_config(
            ConfigOption::string("password", "Password")
                .with_description("Database password (use environment variable)")
                .with_default("${DATABASE_PASSWORD}")
                .in_group("Connection"),
        )
        .with_config(
            ConfigOption::integer("pool_size", "Connection Pool Size")
                .with_description("Maximum number of connections in the pool")
                .with_default(imortal_core::ConfigValue::Int(10))
                .with_min(1.0)
                .with_max(100.0)
                .in_group("Pool"),
        )
        .with_config(
            ConfigOption::integer("pool_min", "Minimum Pool Size")
                .with_description("Minimum number of connections to maintain")
                .with_default(imortal_core::ConfigValue::Int(1))
                .with_min(0.0)
                .in_group("Pool"),
        )
        .with_config(
            ConfigOption::integer("connection_timeout", "Connection Timeout (ms)")
                .with_description("Timeout for establishing connections")
                .with_default(imortal_core::ConfigValue::Int(5000))
                .in_group("Timeouts"),
        )
        .with_config(
            ConfigOption::integer("query_timeout", "Query Timeout (ms)")
                .with_description("Default timeout for queries (0 = no timeout)")
                .with_default(imortal_core::ConfigValue::Int(30000))
                .in_group("Timeouts"),
        )
        .with_config(
            ConfigOption::boolean("ssl", "Enable SSL")
                .with_description("Use SSL/TLS for database connections")
                .with_default(true)
                .in_group("Security"),
        )
        .with_config(
            ConfigOption::boolean("auto_migrate", "Auto Migrate")
                .with_description("Automatically run migrations on startup")
                .with_default(false)
                .advanced(),
        )
        .with_config(
            ConfigOption::boolean("log_queries", "Log Queries")
                .with_description("Log all SQL queries (for debugging)")
                .with_default(false)
                .advanced(),
        )
        .with_default_size(200.0, 160.0)
        .with_generator("storage::database")
        .with_instance_limits(1, 0) // At least one database, unlimited max
}

/// Create the Cache component definition
///
/// In-memory or distributed caching for improved performance
pub fn cache_component() -> ComponentDefinition {
    ComponentDefinition::new("storage.cache", "Cache", ComponentCategory::Storage)
        .with_description("In-memory or distributed caching for performance optimization")
        .with_icon("⚡")
        .with_tag("performance")
        .with_tag("memory")
        .with_tag("redis")
        // Input ports
        .with_input(
            PortDefinition::data_in("key", "Key", DataType::String)
                .with_description("Cache key to get/set"),
        )
        .with_input(
            PortDefinition::data_in("value", "Value", DataType::Any)
                .with_description("Value to cache"),
        )
        .with_input(
            PortDefinition::data_in("ttl", "TTL", DataType::Int64)
                .with_description("Time-to-live in seconds (overrides default)"),
        )
        .with_input(
            PortDefinition::trigger_in("get", "Get")
                .with_description("Get a value from cache"),
        )
        .with_input(
            PortDefinition::trigger_in("set", "Set")
                .with_description("Set a value in cache"),
        )
        .with_input(
            PortDefinition::trigger_in("delete", "Delete")
                .with_description("Delete a value from cache"),
        )
        .with_input(
            PortDefinition::trigger_in("clear", "Clear")
                .with_description("Clear all cached values"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("result", "Result", DataType::Any)
                .with_description("Retrieved value from cache"),
        )
        .with_output(
            PortDefinition::data_out("exists", "Exists", DataType::Bool)
                .with_description("Whether the key exists in cache"),
        )
        .with_output(
            PortDefinition::trigger_out("on_hit", "Cache Hit")
                .with_description("Triggered when value is found in cache"),
        )
        .with_output(
            PortDefinition::trigger_out("on_miss", "Cache Miss")
                .with_description("Triggered when value is not in cache"),
        )
        .with_output(
            PortDefinition::trigger_out("on_set", "On Set")
                .with_description("Triggered when value is successfully cached"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when an error occurs"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message if operation fails"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("backend", "Cache Backend")
                .with_option("memory", "In-Memory")
                .with_option("redis", "Redis")
                .with_option("memcached", "Memcached")
                .with_default("memory")
                .required()
                .with_description("Cache storage backend"),
        )
        .with_config(
            ConfigOption::string("redis_url", "Redis URL")
                .with_description("Redis connection URL")
                .with_default("redis://localhost:6379")
                .in_group("Redis"),
        )
        .with_config(
            ConfigOption::integer("redis_db", "Redis Database")
                .with_description("Redis database number")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_min(0.0)
                .with_max(15.0)
                .in_group("Redis"),
        )
        .with_config(
            ConfigOption::integer("default_ttl", "Default TTL (seconds)")
                .with_description("Default time-to-live for cached items")
                .with_default(imortal_core::ConfigValue::Int(3600))
                .with_min(0.0),
        )
        .with_config(
            ConfigOption::integer("max_memory_mb", "Max Memory (MB)")
                .with_description("Maximum memory usage for in-memory cache")
                .with_default(imortal_core::ConfigValue::Int(128))
                .with_min(1.0)
                .in_group("Memory"),
        )
        .with_config(
            ConfigOption::select("eviction_policy", "Eviction Policy")
                .with_option("lru", "Least Recently Used (LRU)")
                .with_option("lfu", "Least Frequently Used (LFU)")
                .with_option("ttl", "TTL-based")
                .with_option("random", "Random")
                .with_default("lru")
                .with_description("Policy for evicting items when cache is full")
                .in_group("Memory"),
        )
        .with_config(
            ConfigOption::string("key_prefix", "Key Prefix")
                .with_description("Prefix for all cache keys (useful for namespacing)")
                .with_default(""),
        )
        .with_config(
            ConfigOption::boolean("compression", "Enable Compression")
                .with_description("Compress cached values to save memory")
                .with_default(false)
                .advanced(),
        )
        .with_config(
            ConfigOption::boolean("stats", "Enable Statistics")
                .with_description("Track cache hit/miss statistics")
                .with_default(true)
                .advanced(),
        )
        .with_default_size(180.0, 140.0)
        .with_generator("storage::cache")
}

/// Create the FileStorage component definition
///
/// File/blob storage for documents, images, and other binary data
pub fn file_storage_component() -> ComponentDefinition {
    ComponentDefinition::new("storage.files", "File Storage", ComponentCategory::Storage)
        .with_description("File and blob storage for documents, images, and binary data")
        .with_icon("📁")
        .with_tag("files")
        .with_tag("blob")
        .with_tag("uploads")
        .with_tag("s3")
        // Input ports
        .with_input(
            PortDefinition::data_in("file", "File", DataType::Bytes)
                .with_description("File data to upload"),
        )
        .with_input(
            PortDefinition::data_in("path", "Path", DataType::String)
                .with_description("File path/key"),
        )
        .with_input(
            PortDefinition::data_in("metadata", "Metadata", DataType::Json)
                .with_description("File metadata (content type, etc.)"),
        )
        .with_input(
            PortDefinition::trigger_in("upload", "Upload")
                .with_description("Upload a file"),
        )
        .with_input(
            PortDefinition::trigger_in("download", "Download")
                .with_description("Download a file"),
        )
        .with_input(
            PortDefinition::trigger_in("delete", "Delete")
                .with_description("Delete a file"),
        )
        .with_input(
            PortDefinition::trigger_in("list", "List")
                .with_description("List files in a directory"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("data", "Data", DataType::Bytes)
                .with_description("Downloaded file data"),
        )
        .with_output(
            PortDefinition::data_out("url", "URL", DataType::String)
                .with_description("URL to access the file"),
        )
        .with_output(
            PortDefinition::data_out("signed_url", "Signed URL", DataType::String)
                .with_description("Temporary signed URL for secure access"),
        )
        .with_output(
            PortDefinition::data_out("file_info", "File Info", DataType::Json)
                .with_description("File metadata and information"),
        )
        .with_output(
            PortDefinition::data_out("files", "Files", DataType::Array(Box::new(DataType::Json)))
                .with_description("List of files from directory listing"),
        )
        .with_output(
            PortDefinition::trigger_out("on_upload", "On Upload")
                .with_description("Triggered when upload completes"),
        )
        .with_output(
            PortDefinition::trigger_out("on_download", "On Download")
                .with_description("Triggered when download completes"),
        )
        .with_output(
            PortDefinition::trigger_out("on_delete", "On Delete")
                .with_description("Triggered when file is deleted"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when an error occurs"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message if operation fails"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("backend", "Storage Backend")
                .with_option("local", "Local Filesystem")
                .with_option("s3", "Amazon S3")
                .with_option("gcs", "Google Cloud Storage")
                .with_option("azure", "Azure Blob Storage")
                .with_option("minio", "MinIO")
                .with_default("local")
                .required()
                .with_description("Storage backend to use"),
        )
        .with_config(
            ConfigOption::string("base_path", "Base Path")
                .with_description("Base path/bucket for file storage")
                .with_default("./storage"),
        )
        .with_config(
            ConfigOption::string("bucket", "Bucket Name")
                .with_description("S3/GCS/Azure bucket name")
                .in_group("Cloud"),
        )
        .with_config(
            ConfigOption::string("region", "Region")
                .with_description("Cloud storage region")
                .with_default("us-east-1")
                .in_group("Cloud"),
        )
        .with_config(
            ConfigOption::string("access_key", "Access Key")
                .with_description("Cloud storage access key (use environment variable)")
                .with_default("${STORAGE_ACCESS_KEY}")
                .in_group("Cloud"),
        )
        .with_config(
            ConfigOption::string("secret_key", "Secret Key")
                .with_description("Cloud storage secret key (use environment variable)")
                .with_default("${STORAGE_SECRET_KEY}")
                .in_group("Cloud"),
        )
        .with_config(
            ConfigOption::string("endpoint", "Custom Endpoint")
                .with_description("Custom endpoint URL (for MinIO, etc.)")
                .in_group("Cloud")
                .advanced(),
        )
        .with_config(
            ConfigOption::integer("max_file_size_mb", "Max File Size (MB)")
                .with_description("Maximum allowed file size in megabytes")
                .with_default(imortal_core::ConfigValue::Int(100))
                .with_min(1.0)
                .with_max(5000.0)
                .in_group("Limits"),
        )
        .with_config(
            ConfigOption::string("allowed_types", "Allowed File Types")
                .with_description("Comma-separated list of allowed MIME types (empty = all)")
                .with_default("")
                .in_group("Limits"),
        )
        .with_config(
            ConfigOption::integer("signed_url_expiry", "Signed URL Expiry (seconds)")
                .with_description("How long signed URLs remain valid")
                .with_default(imortal_core::ConfigValue::Int(3600))
                .with_min(60.0)
                .in_group("Security"),
        )
        .with_config(
            ConfigOption::boolean("public_read", "Public Read")
                .with_description("Allow public read access to uploaded files")
                .with_default(false)
                .in_group("Security"),
        )
        .with_config(
            ConfigOption::boolean("versioning", "Enable Versioning")
                .with_description("Keep previous versions of files")
                .with_default(false)
                .advanced(),
        )
        .with_default_size(200.0, 160.0)
        .with_generator("storage::files")
}

/// Create a KeyValue Store component definition
///
/// Simple key-value storage for application state
pub fn kv_store_component() -> ComponentDefinition {
    ComponentDefinition::new("storage.kv", "Key-Value Store", ComponentCategory::Storage)
        .with_description("Simple key-value storage for application state and configuration")
        .with_icon("🗄")
        .with_tag("state")
        .with_tag("config")
        .with_tag("settings")
        // Input ports
        .with_input(
            PortDefinition::data_in("key", "Key", DataType::String)
                .with_description("Key to get/set")
                .required(),
        )
        .with_input(
            PortDefinition::data_in("value", "Value", DataType::Any)
                .with_description("Value to store"),
        )
        .with_input(
            PortDefinition::trigger_in("get", "Get")
                .with_description("Get a value by key"),
        )
        .with_input(
            PortDefinition::trigger_in("set", "Set")
                .with_description("Set a value"),
        )
        .with_input(
            PortDefinition::trigger_in("delete", "Delete")
                .with_description("Delete a key"),
        )
        .with_input(
            PortDefinition::trigger_in("keys", "Get Keys")
                .with_description("Get all keys matching a pattern"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("result", "Result", DataType::Any)
                .with_description("Retrieved value"),
        )
        .with_output(
            PortDefinition::data_out("keys", "Keys", DataType::Array(Box::new(DataType::String)))
                .with_description("List of keys"),
        )
        .with_output(
            PortDefinition::trigger_out("on_success", "On Success")
                .with_description("Triggered when operation succeeds"),
        )
        .with_output(
            PortDefinition::trigger_out("on_not_found", "Not Found")
                .with_description("Triggered when key is not found"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when an error occurs"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("backend", "Storage Backend")
                .with_option("memory", "In-Memory")
                .with_option("file", "File-based")
                .with_option("database", "Database")
                .with_option("redis", "Redis")
                .with_default("file"),
        )
        .with_config(
            ConfigOption::string("file_path", "Storage File Path")
                .with_description("Path for file-based storage")
                .with_default("./data/kv.json")
                .in_group("File"),
        )
        .with_config(
            ConfigOption::boolean("persist", "Persist Changes")
                .with_description("Persist changes to disk immediately")
                .with_default(true)
                .in_group("File"),
        )
        .with_config(
            ConfigOption::string("namespace", "Namespace")
                .with_description("Namespace/prefix for all keys")
                .with_default(""),
        )
        .with_default_size(180.0, 130.0)
        .with_generator("storage::kv")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_component() {
        let def = database_component();
        assert_eq!(def.id, "storage.database");
        assert_eq!(def.category, ComponentCategory::Storage);
        assert!(def.config.iter().any(|c| c.id == "backend"));
        assert!(def.config.iter().any(|c| c.id == "pool_size"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "connection"));
    }

    #[test]
    fn test_cache_component() {
        let def = cache_component();
        assert_eq!(def.id, "storage.cache");
        assert!(def.config.iter().any(|c| c.id == "backend"));
        assert!(def.ports.inputs.iter().any(|p| p.id == "get"));
        assert!(def.ports.inputs.iter().any(|p| p.id == "set"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "on_hit"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "on_miss"));
    }

    #[test]
    fn test_file_storage_component() {
        let def = file_storage_component();
        assert_eq!(def.id, "storage.files");
        assert!(def.config.iter().any(|c| c.id == "backend"));
        assert!(def.ports.inputs.iter().any(|p| p.id == "upload"));
        assert!(def.ports.inputs.iter().any(|p| p.id == "download"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "url"));
    }

    #[test]
    fn test_kv_store_component() {
        let def = kv_store_component();
        assert_eq!(def.id, "storage.kv");
        assert!(def.ports.inputs.iter().any(|p| p.id == "get"));
        assert!(def.ports.inputs.iter().any(|p| p.id == "set"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "result"));
    }

    #[test]
    fn test_database_instantiation() {
        let def = database_component();
        let node = def.instantiate("MainDB");

        assert_eq!(node.name, "MainDB");
        assert_eq!(node.component_type, "storage.database");
        assert!(node.config.contains_key("backend"));
    }
}
