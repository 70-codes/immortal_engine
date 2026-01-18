//! Rust handler code generation
//!
//! This module provides utilities for generating Rust handler functions
//! from API and route components.

use imortal_ir::Node;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

/// Generate an HTTP handler function for a REST endpoint node
pub fn generate_rest_handler(node: &Node) -> String {
    let handler_name = format_ident!("{}", to_snake_case(&node.name));

    // Get configuration
    let method = node.get_config_str("method").unwrap_or("GET");
    let path = node.fields.iter()
        .find(|f| f.name == "path")
        .and_then(|f| f.default_value.as_ref())
        .and_then(|v| v.as_str())
        .unwrap_or("/");

    let tokens = match method.to_uppercase().as_str() {
        "GET" => generate_get_handler(&handler_name, path),
        "POST" => generate_post_handler(&handler_name, path),
        "PUT" => generate_put_handler(&handler_name, path),
        "DELETE" => generate_delete_handler(&handler_name, path),
        _ => generate_get_handler(&handler_name, path),
    };

    tokens.to_string()
}

fn generate_get_handler(name: &proc_macro2::Ident, _path: &str) -> TokenStream {
    quote! {
        pub async fn #name() -> impl axum::response::IntoResponse {
            // TODO: Implement handler logic
            axum::Json(serde_json::json!({"status": "ok"}))
        }
    }
}

fn generate_post_handler(name: &proc_macro2::Ident, _path: &str) -> TokenStream {
    quote! {
        pub async fn #name(
            axum::Json(payload): axum::Json<serde_json::Value>,
        ) -> impl axum::response::IntoResponse {
            // TODO: Implement handler logic
            axum::Json(serde_json::json!({"status": "created", "data": payload}))
        }
    }
}

fn generate_put_handler(name: &proc_macro2::Ident, _path: &str) -> TokenStream {
    quote! {
        pub async fn #name(
            axum::extract::Path(id): axum::extract::Path<String>,
            axum::Json(payload): axum::Json<serde_json::Value>,
        ) -> impl axum::response::IntoResponse {
            // TODO: Implement handler logic
            axum::Json(serde_json::json!({"status": "updated", "id": id, "data": payload}))
        }
    }
}

fn generate_delete_handler(name: &proc_macro2::Ident, _path: &str) -> TokenStream {
    quote! {
        pub async fn #name(
            axum::extract::Path(id): axum::extract::Path<String>,
        ) -> impl axum::response::IntoResponse {
            // TODO: Implement handler logic
            axum::Json(serde_json::json!({"status": "deleted", "id": id}))
        }
    }
}

/// Generate router configuration for all API endpoints
pub fn generate_router(nodes: &[&Node]) -> String {
    let routes: Vec<TokenStream> = nodes.iter()
        .filter(|n| n.component_type == "api.rest")
        .map(|node| {
            let handler_name = format_ident!("{}", to_snake_case(&node.name));
            let method = node.get_config_str("method").unwrap_or("GET");
            let path = node.fields.iter()
                .find(|f| f.name == "path")
                .and_then(|f| f.default_value.as_ref())
                .and_then(|v| v.as_str())
                .unwrap_or("/");

            match method.to_uppercase().as_str() {
                "GET" => quote! { .route(#path, axum::routing::get(#handler_name)) },
                "POST" => quote! { .route(#path, axum::routing::post(#handler_name)) },
                "PUT" => quote! { .route(#path, axum::routing::put(#handler_name)) },
                "DELETE" => quote! { .route(#path, axum::routing::delete(#handler_name)) },
                "PATCH" => quote! { .route(#path, axum::routing::patch(#handler_name)) },
                _ => quote! { .route(#path, axum::routing::get(#handler_name)) },
            }
        })
        .collect();

    let tokens = quote! {
        pub fn create_router() -> axum::Router {
            axum::Router::new()
                #(#routes)*
        }
    };

    tokens.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("GetUsers"), "get_users");
        assert_eq!(to_snake_case("createUser"), "create_user");
        assert_eq!(to_snake_case("delete-item"), "delete_item");
    }
}
