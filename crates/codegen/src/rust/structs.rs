//! Rust struct generation utilities
//!
//! This module provides utilities for generating Rust struct definitions
//! from IR nodes.

use imortal_ir::{Node, Field};
use imortal_core::DataType;

/// Generate a Rust struct definition from a node
pub fn generate_struct_definition(node: &Node) -> String {
    let mut output = String::new();

    // Add doc comment
    if let Some(desc) = &node.description {
        output.push_str(&format!("/// {}\n", desc));
    }

    // Add derives
    output.push_str("#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\n");

    // Start struct
    output.push_str(&format!("pub struct {} {{\n", node.name));

    // Add fields
    for field in &node.fields {
        output.push_str(&generate_field_definition(field));
    }

    output.push_str("}\n");

    output
}

/// Generate a single field definition
fn generate_field_definition(field: &Field) -> String {
    let mut output = String::new();

    // Add doc comment if description exists
    if let Some(desc) = &field.description {
        output.push_str(&format!("    /// {}\n", desc));
    }

    // Add serde attributes if needed
    if field.ui_hints.secret {
        output.push_str("    #[serde(skip_serializing)]\n");
    }

    // Get the Rust type
    let rust_type = field.rust_type();

    // Generate the field line
    output.push_str(&format!("    pub {}: {},\n", to_snake_case(&field.name), rust_type));

    output
}

/// Generate an impl block with common methods
pub fn generate_impl_block(node: &Node) -> String {
    let mut output = String::new();

    output.push_str(&format!("impl {} {{\n", node.name));

    // Generate new() constructor
    output.push_str("    /// Create a new instance\n");
    output.push_str("    pub fn new() -> Self {\n");
    output.push_str("        Self::default()\n");
    output.push_str("    }\n");

    output.push_str("}\n\n");

    // Generate Default impl
    output.push_str(&format!("impl Default for {} {{\n", node.name));
    output.push_str("    fn default() -> Self {\n");
    output.push_str("        Self {\n");

    for field in &node.fields {
        let default_value = get_default_value(field);
        output.push_str(&format!("            {}: {},\n", to_snake_case(&field.name), default_value));
    }

    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    output
}

/// Get the default value for a field
fn get_default_value(field: &Field) -> String {
    if let Some(default) = &field.default_value {
        match default {
            imortal_core::ConfigValue::String(s) => format!("\"{}\".to_string()", s),
            imortal_core::ConfigValue::Int(i) => i.to_string(),
            imortal_core::ConfigValue::Float(f) => format!("{}f64", f),
            imortal_core::ConfigValue::Bool(b) => b.to_string(),
            _ => "Default::default()".to_string(),
        }
    } else if !field.required {
        "None".to_string()
    } else {
        match &field.data_type {
            DataType::String | DataType::Text => "String::new()".to_string(),
            DataType::Int32 | DataType::Int64 => "0".to_string(),
            DataType::Float32 | DataType::Float64 => "0.0".to_string(),
            DataType::Bool => "false".to_string(),
            DataType::Uuid => "uuid::Uuid::new_v4()".to_string(),
            DataType::Array(_) => "Vec::new()".to_string(),
            _ => "Default::default()".to_string(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("user_name"), "user_name");
        assert_eq!(to_snake_case("ID"), "id");
    }
}
