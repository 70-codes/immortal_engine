//! API Component Definitions
//!
//! This module provides component definitions for API-related functionality:
//! - REST Endpoint: RESTful API endpoints
//! - GraphQL Endpoint: GraphQL API endpoints
//! - WebSocket: WebSocket connections

use crate::definition::{
    ComponentDefinition, ConfigOption, ConfigType, FieldDefinition, PortDefinition,
};
use imortal_core::{ComponentCategory, DataType, Validation};

/// Create the REST Endpoint component definition
pub fn rest_endpoint_component() -> ComponentDefinition {
    ComponentDefinition::new("api.rest", "REST Endpoint", ComponentCategory::Api)
        .with_description("Define a RESTful API endpoint with HTTP methods")
        .with_icon("🔌")
        .with_tag("http")
        .with_tag("rest")
        .with_tag("api")
        // Fields
        .with_field(
            FieldDefinition::string("path")
                .required()
                .with_label("Path")
                .with_placeholder("/api/users/:id")
                .with_description("URL path for this endpoint")
                .with_order(1),
        )
        .with_field(
            FieldDefinition::string("method")
                .required()
                .with_label("Method")
                .with_default("GET")
                .with_order(2),
        )
        // Input ports
        .with_input(
            PortDefinition::data_in("request", "Request", DataType::Any)
                .with_description("Incoming HTTP request"),
        )
        .with_input(
            PortDefinition::data_in("body", "Request Body", DataType::Json)
                .with_description("Request body data"),
        )
        .with_input(
            PortDefinition::data_in("params", "Path Params", DataType::Json)
                .with_description("URL path parameters"),
        )
        .with_input(
            PortDefinition::data_in("query", "Query Params", DataType::Json)
                .with_description("URL query parameters"),
        )
        .with_input(
            PortDefinition::data_in("headers", "Headers", DataType::Json)
                .with_description("Request headers"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("response", "Response", DataType::Any)
                .with_description("HTTP response to send"),
        )
        .with_output(
            PortDefinition::trigger_out("on_request", "On Request")
                .with_description("Triggered when a request is received"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message if request handling fails"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("method", "HTTP Method")
                .with_option("GET", "GET")
                .with_option("POST", "POST")
                .with_option("PUT", "PUT")
                .with_option("PATCH", "PATCH")
                .with_option("DELETE", "DELETE")
                .with_option("HEAD", "HEAD")
                .with_option("OPTIONS", "OPTIONS")
                .with_default("GET")
                .with_description("HTTP method for this endpoint"),
        )
        .with_config(
            ConfigOption::boolean("auth_required", "Require Authentication")
                .with_default(false)
                .with_description("Require authenticated user to access"),
        )
        .with_config(
            ConfigOption::string("roles", "Required Roles")
                .with_description("Comma-separated list of allowed roles")
                .with_default(""),
        )
        .with_config(
            ConfigOption::select("response_type", "Response Type")
                .with_option("json", "JSON")
                .with_option("text", "Plain Text")
                .with_option("html", "HTML")
                .with_option("xml", "XML")
                .with_option("binary", "Binary")
                .with_default("json"),
        )
        .with_config(
            ConfigOption::integer("rate_limit", "Rate Limit")
                .with_description("Requests per minute (0 = unlimited)")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_min(0.0)
                .advanced(),
        )
        .with_config(
            ConfigOption::integer("timeout_ms", "Timeout (ms)")
                .with_description("Request timeout in milliseconds")
                .with_default(imortal_core::ConfigValue::Int(30000))
                .with_min(0.0)
                .advanced(),
        )
        .with_config(
            ConfigOption::boolean("cors_enabled", "Enable CORS")
                .with_default(true)
                .advanced(),
        )
        .with_generator("api::rest")
        .with_default_size(220.0, 180.0)
}

/// Create the GraphQL Endpoint component definition
pub fn graphql_endpoint_component() -> ComponentDefinition {
    ComponentDefinition::new("api.graphql", "GraphQL", ComponentCategory::Api)
        .with_description("Define a GraphQL API with queries, mutations, and subscriptions")
        .with_icon("◈")
        .with_tag("graphql")
        .with_tag("api")
        .with_tag("query")
        // Fields
        .with_field(
            FieldDefinition::string("name")
                .required()
                .with_label("Operation Name")
                .with_placeholder("getUsers")
                .with_order(1),
        )
        .with_field(
            FieldDefinition::text("schema")
                .with_label("Schema Definition")
                .with_placeholder("type Query {\n  users: [User!]!\n}")
                .with_description("GraphQL schema definition")
                .with_order(2),
        )
        // Input ports
        .with_input(
            PortDefinition::data_in("variables", "Variables", DataType::Json)
                .with_description("GraphQL query variables"),
        )
        .with_input(
            PortDefinition::data_in("context", "Context", DataType::Any)
                .with_description("Request context (auth, etc)"),
        )
        .with_input(
            PortDefinition::trigger_in("execute", "Execute")
                .with_description("Execute the GraphQL operation"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("data", "Data", DataType::Json)
                .with_description("Query result data"),
        )
        .with_output(
            PortDefinition::data_out("errors", "Errors", DataType::Array(Box::new(DataType::Json)))
                .with_description("GraphQL errors"),
        )
        .with_output(
            PortDefinition::trigger_out("on_query", "On Query")
                .with_description("Triggered when a query is received"),
        )
        .with_output(
            PortDefinition::trigger_out("on_mutation", "On Mutation")
                .with_description("Triggered when a mutation is received"),
        )
        .with_output(
            PortDefinition::trigger_out("on_subscription", "On Subscription")
                .with_description("Triggered when a subscription is started"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("operation_type", "Operation Type")
                .with_option("query", "Query")
                .with_option("mutation", "Mutation")
                .with_option("subscription", "Subscription")
                .with_default("query"),
        )
        .with_config(
            ConfigOption::string("path", "Endpoint Path")
                .with_default("/graphql")
                .with_description("URL path for the GraphQL endpoint"),
        )
        .with_config(
            ConfigOption::boolean("introspection", "Enable Introspection")
                .with_default(true)
                .with_description("Allow GraphQL introspection queries"),
        )
        .with_config(
            ConfigOption::boolean("playground", "Enable Playground")
                .with_default(true)
                .with_description("Enable GraphQL playground/explorer"),
        )
        .with_config(
            ConfigOption::integer("max_depth", "Max Query Depth")
                .with_description("Maximum nested query depth (0 = unlimited)")
                .with_default(imortal_core::ConfigValue::Int(10))
                .with_min(0.0)
                .advanced(),
        )
        .with_config(
            ConfigOption::integer("max_complexity", "Max Complexity")
                .with_description("Maximum query complexity (0 = unlimited)")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_min(0.0)
                .advanced(),
        )
        .with_generator("api::graphql")
        .with_default_size(220.0, 200.0)
}

/// Create the WebSocket component definition
pub fn websocket_component() -> ComponentDefinition {
    ComponentDefinition::new("api.websocket", "WebSocket", ComponentCategory::Api)
        .with_description("WebSocket connection for real-time bidirectional communication")
        .with_icon("🔄")
        .with_tag("websocket")
        .with_tag("realtime")
        .with_tag("socket")
        // Fields
        .with_field(
            FieldDefinition::string("path")
                .required()
                .with_label("Path")
                .with_placeholder("/ws")
                .with_description("WebSocket endpoint path")
                .with_order(1),
        )
        .with_field(
            FieldDefinition::string("channel")
                .with_label("Channel")
                .with_placeholder("notifications")
                .with_description("Channel/room name for grouping connections")
                .with_order(2),
        )
        // Input ports
        .with_input(
            PortDefinition::data_in("message", "Send Message", DataType::Any)
                .with_description("Message to send to connected clients"),
        )
        .with_input(
            PortDefinition::data_in("broadcast", "Broadcast", DataType::Any)
                .with_description("Message to broadcast to all clients"),
        )
        .with_input(
            PortDefinition::trigger_in("close", "Close Connection")
                .with_description("Close the WebSocket connection"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("received", "Received", DataType::Any)
                .with_description("Message received from client"),
        )
        .with_output(
            PortDefinition::data_out("client_id", "Client ID", DataType::String)
                .with_description("Unique identifier for the connected client"),
        )
        .with_output(
            PortDefinition::trigger_out("on_connect", "On Connect")
                .with_description("Triggered when a client connects"),
        )
        .with_output(
            PortDefinition::trigger_out("on_message", "On Message")
                .with_description("Triggered when a message is received"),
        )
        .with_output(
            PortDefinition::trigger_out("on_disconnect", "On Disconnect")
                .with_description("Triggered when a client disconnects"),
        )
        .with_output(
            PortDefinition::trigger_out("on_error", "On Error")
                .with_description("Triggered when an error occurs"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("auth_required", "Require Authentication")
                .with_default(false)
                .with_description("Require authenticated connection"),
        )
        .with_config(
            ConfigOption::select("message_format", "Message Format")
                .with_option("json", "JSON")
                .with_option("text", "Plain Text")
                .with_option("binary", "Binary")
                .with_default("json"),
        )
        .with_config(
            ConfigOption::integer("ping_interval", "Ping Interval (s)")
                .with_description("Seconds between ping messages (0 = disabled)")
                .with_default(imortal_core::ConfigValue::Int(30))
                .with_min(0.0),
        )
        .with_config(
            ConfigOption::integer("max_connections", "Max Connections")
                .with_description("Maximum concurrent connections (0 = unlimited)")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_min(0.0)
                .advanced(),
        )
        .with_config(
            ConfigOption::integer("max_message_size", "Max Message Size")
                .with_description("Maximum message size in bytes")
                .with_default(imortal_core::ConfigValue::Int(65536))
                .with_min(1024.0)
                .advanced(),
        )
        .with_config(
            ConfigOption::boolean("compression", "Enable Compression")
                .with_default(false)
                .advanced(),
        )
        .with_generator("api::websocket")
        .with_default_size(200.0, 180.0)
}

/// Create a Middleware component definition
pub fn middleware_component() -> ComponentDefinition {
    ComponentDefinition::new("api.middleware", "Middleware", ComponentCategory::Api)
        .with_description("Request/response middleware for API processing")
        .with_icon("⚡")
        .with_tag("middleware")
        .with_tag("filter")
        .with_tag("interceptor")
        // Input ports
        .with_input(
            PortDefinition::data_in("request", "Request", DataType::Any)
                .with_description("Incoming request to process"),
        )
        .with_input(
            PortDefinition::data_in("response", "Response", DataType::Any)
                .with_description("Response to process"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("next_request", "Modified Request", DataType::Any)
                .with_description("Request passed to next handler"),
        )
        .with_output(
            PortDefinition::data_out("next_response", "Modified Response", DataType::Any)
                .with_description("Response passed to client"),
        )
        .with_output(
            PortDefinition::trigger_out("on_request", "On Request")
                .with_description("Triggered for each incoming request"),
        )
        .with_output(
            PortDefinition::trigger_out("on_response", "On Response")
                .with_description("Triggered for each outgoing response"),
        )
        .with_output(
            PortDefinition::trigger_out("reject", "Reject")
                .with_description("Triggered when request should be rejected"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("type", "Middleware Type")
                .with_option("auth", "Authentication")
                .with_option("logging", "Logging")
                .with_option("cors", "CORS")
                .with_option("rate_limit", "Rate Limiting")
                .with_option("compression", "Compression")
                .with_option("custom", "Custom")
                .with_default("custom"),
        )
        .with_config(
            ConfigOption::integer("priority", "Priority")
                .with_description("Execution order (lower = earlier)")
                .with_default(imortal_core::ConfigValue::Int(100)),
        )
        .with_config(
            ConfigOption::string("path_pattern", "Path Pattern")
                .with_description("Apply to paths matching this pattern (regex)")
                .with_default(".*"),
        )
        .with_config(
            ConfigOption::string("exclude_paths", "Exclude Paths")
                .with_description("Comma-separated paths to exclude")
                .with_default(""),
        )
        .with_generator("api::middleware")
        .with_default_size(180.0, 140.0)
}

/// Create an API Router component definition
pub fn router_component() -> ComponentDefinition {
    ComponentDefinition::new("api.router", "API Router", ComponentCategory::Api)
        .with_description("Route incoming requests to appropriate handlers")
        .with_icon("🔀")
        .with_tag("router")
        .with_tag("routing")
        // Input ports
        .with_input(
            PortDefinition::data_in("request", "Request", DataType::Any)
                .with_description("Incoming request to route")
                .required(),
        )
        // Output ports - dynamic based on routes, but we define common ones
        .with_output(
            PortDefinition::data_out("matched_route", "Matched Route", DataType::String)
                .with_description("The matched route pattern"),
        )
        .with_output(
            PortDefinition::data_out("route_params", "Route Params", DataType::Json)
                .with_description("Extracted route parameters"),
        )
        .with_output(
            PortDefinition::trigger_out("on_match", "On Match")
                .with_description("Triggered when a route matches"),
        )
        .with_output(
            PortDefinition::trigger_out("not_found", "Not Found")
                .with_description("Triggered when no route matches"),
        )
        // Configuration
        .with_config(
            ConfigOption::string("base_path", "Base Path")
                .with_description("Base path prefix for all routes")
                .with_default("/api"),
        )
        .with_config(
            ConfigOption::boolean("trailing_slash", "Trailing Slash")
                .with_description("How to handle trailing slashes")
                .with_default(false),
        )
        .with_config(
            ConfigOption::boolean("case_sensitive", "Case Sensitive")
                .with_description("Case sensitive route matching")
                .with_default(true),
        )
        .with_generator("api::router")
        .with_default_size(180.0, 140.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest_endpoint_component() {
        let component = rest_endpoint_component();

        assert_eq!(component.id, "api.rest");
        assert_eq!(component.category, ComponentCategory::Api);
        assert!(component.fields.iter().any(|f| f.name == "path"));
        assert!(component.fields.iter().any(|f| f.name == "method"));
        assert!(component.config.iter().any(|c| c.id == "method"));
    }

    #[test]
    fn test_graphql_endpoint_component() {
        let component = graphql_endpoint_component();

        assert_eq!(component.id, "api.graphql");
        assert!(component.ports.outputs.iter().any(|p| p.id == "data"));
        assert!(component.config.iter().any(|c| c.id == "operation_type"));
    }

    #[test]
    fn test_websocket_component() {
        let component = websocket_component();

        assert_eq!(component.id, "api.websocket");
        assert!(component.ports.inputs.iter().any(|p| p.id == "message"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "on_connect"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "on_message"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "on_disconnect"));
    }

    #[test]
    fn test_middleware_component() {
        let component = middleware_component();

        assert_eq!(component.id, "api.middleware");
        assert!(component.config.iter().any(|c| c.id == "type"));
        assert!(component.config.iter().any(|c| c.id == "priority"));
    }

    #[test]
    fn test_router_component() {
        let component = router_component();

        assert_eq!(component.id, "api.router");
        assert!(component.ports.outputs.iter().any(|p| p.id == "not_found"));
    }

    #[test]
    fn test_instantiate_rest() {
        let component = rest_endpoint_component();
        let node = component.instantiate("Get Users");

        assert_eq!(node.name, "Get Users");
        assert_eq!(node.component_type, "api.rest");
        assert!(node.fields.iter().any(|f| f.name == "path"));
    }
}
