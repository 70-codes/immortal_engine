//! Rust code generation module
//!
//! This module provides Rust-specific code generation utilities.

pub mod structs;
pub mod handlers;
pub mod models;
pub mod migrations;
pub mod auth;
pub mod config;

// Re-export common types
pub use structs::*;
pub use handlers::*;
pub use models::*;
pub use migrations::{Migration, MigrationConfig, MigrationGenerator, DatabaseBackend};
pub use auth::{AuthConfig, AuthFramework, AuthGenerator, GeneratedAuth};
pub use config::{generate_config, generate_error};

use imortal_ir::Node;
use imortal_core::DataType;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

/// Generate Rust struct code from a Node
pub fn generate_struct(node: &Node) -> String {
    let struct_name = format_ident!("{}", to_pascal_case(&node.name));

    let fields: Vec<TokenStream> = node.fields.iter().map(|field| {
        let field_name = format_ident!("{}", to_snake_case(&field.name));
        let field_type = data_type_to_rust(&field.data_type, !field.required);

        quote! {
            pub #field_name: #field_type,
        }
    }).collect();

    let tokens = quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct #struct_name {
            #(#fields)*
        }
    };

    tokens.to_string()
}

/// Convert a DataType to Rust type TokenStream
pub fn data_type_to_rust(data_type: &DataType, optional: bool) -> TokenStream {
    let base_type = match data_type {
        DataType::String | DataType::Text => quote! { String },
        DataType::Int32 => quote! { i32 },
        DataType::Int64 => quote! { i64 },
        DataType::Float32 => quote! { f32 },
        DataType::Float64 => quote! { f64 },
        DataType::Bool => quote! { bool },
        DataType::Uuid => quote! { uuid::Uuid },
        DataType::DateTime => quote! { chrono::DateTime<chrono::Utc> },
        DataType::Date => quote! { chrono::NaiveDate },
        DataType::Time => quote! { chrono::NaiveTime },
        DataType::Bytes => quote! { Vec<u8> },
        DataType::Json => quote! { serde_json::Value },
        DataType::Optional(inner) => {
            let inner_type = data_type_to_rust(inner, false);
            return quote! { Option<#inner_type> };
        }
        DataType::Array(inner) => {
            let inner_type = data_type_to_rust(inner, false);
            quote! { Vec<#inner_type> }
        }
        DataType::Map { key, value } => {
            let key_type = data_type_to_rust(key, false);
            let value_type = data_type_to_rust(value, false);
            quote! { std::collections::HashMap<#key_type, #value_type> }
        }
        DataType::Reference(entity) | DataType::Entity(entity) => {
            let ident = format_ident!("{}", entity);
            quote! { #ident }
        }
        DataType::Any => quote! { serde_json::Value },
        DataType::Trigger => quote! { () },
        DataType::Custom { type_name, .. } => {
            let ident = format_ident!("{}", type_name);
            quote! { #ident }
        }
    };

    if optional {
        quote! { Option<#base_type> }
    } else {
        base_type
    }
}

/// Convert string to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

/// Convert string to snake_case
pub fn to_snake_case(s: &str) -> String {
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

/// Convert string to camelCase
pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Convert string to SCREAMING_SNAKE_CASE
pub fn to_screaming_snake_case(s: &str) -> String {
    to_snake_case(s).to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("user-profile"), "UserProfile");
        assert_eq!(to_pascal_case("my_api_key"), "MyApiKey");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("UserProfile"), "user_profile");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("User"), "user");
        assert_eq!(to_camel_case("user_profile"), "userProfile");
    }

    #[test]
    fn test_to_screaming_snake_case() {
        assert_eq!(to_screaming_snake_case("HelloWorld"), "HELLO_WORLD");
        assert_eq!(to_screaming_snake_case("api_key"), "API_KEY");
    }

    #[test]
    fn test_data_type_to_rust() {
        let string_type = data_type_to_rust(&DataType::String, false);
        assert_eq!(string_type.to_string(), "String");

        let optional_string = data_type_to_rust(&DataType::String, true);
        assert!(optional_string.to_string().contains("Option"));

        let array_type = data_type_to_rust(&DataType::Array(Box::new(DataType::Int32)), false);
        assert!(array_type.to_string().contains("Vec"));
    }
}
