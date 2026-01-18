//! Core types used throughout Immortal Engine
//!
//! These types form the foundation of the type system and are used by
//! the IR, components, and code generation systems.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for nodes, edges, and other entities
pub type NodeId = Uuid;
pub type EdgeId = Uuid;
pub type PortId = String;
pub type ComponentTypeId = String;

/// Position on the 2D canvas
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::zero()
    }
}

/// Size of a component on the canvas
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn default_component() -> Self {
        Self {
            width: 200.0,
            height: 150.0,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::default_component()
    }
}

/// Bounding rectangle for components
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub position: Position,
    pub size: Size,
}

impl Rect {
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }

    pub fn contains(&self, point: Position) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.size.width
            && point.y >= self.position.y
            && point.y <= self.position.y + self.size.height
    }

    pub fn center(&self) -> Position {
        Position {
            x: self.position.x + self.size.width / 2.0,
            y: self.position.y + self.size.height / 2.0,
        }
    }
}

/// Data types supported by the engine
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum DataType {
    // Primitive types
    String,
    Text, // Long text/content
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    Uuid,
    DateTime,
    Date,
    Time,
    Bytes,
    Json,

    // Complex types
    Optional(Box<DataType>),
    Array(Box<DataType>),
    Map {
        key: Box<DataType>,
        value: Box<DataType>,
    },

    // Reference to another entity/component
    Reference(String),

    // Entity type (for ports that pass entity data)
    Entity(String),

    // Any type (for flexible connections)
    Any,

    // Trigger/signal type (no data, just an event)
    Trigger,

    // Custom domain-specific type
    Custom {
        domain: String,
        type_name: String,
    },
}

impl DataType {
    /// Check if this type can connect to another type
    pub fn is_compatible_with(&self, other: &DataType) -> bool {
        match (self, other) {
            // Any is compatible with everything
            (DataType::Any, _) | (_, DataType::Any) => true,

            // Exact match
            (a, b) if a == b => true,

            // Optional can accept non-optional
            (DataType::Optional(inner), other) if inner.as_ref() == other => true,

            // Array element type matching
            (DataType::Array(a), DataType::Array(b)) => a.is_compatible_with(b),

            // Reference and Entity matching
            (DataType::Reference(a), DataType::Entity(b))
            | (DataType::Entity(a), DataType::Reference(b)) => a == b,

            // Numeric coercion
            (DataType::Int32, DataType::Int64)
            | (DataType::Float32, DataType::Float64)
            | (DataType::Int32, DataType::Float64)
            | (DataType::Int64, DataType::Float64) => true,

            _ => false,
        }
    }

    /// Get the Rust type representation
    pub fn to_rust_type(&self) -> String {
        match self {
            DataType::String | DataType::Text => "String".to_string(),
            DataType::Int32 => "i32".to_string(),
            DataType::Int64 => "i64".to_string(),
            DataType::Float32 => "f32".to_string(),
            DataType::Float64 => "f64".to_string(),
            DataType::Bool => "bool".to_string(),
            DataType::Uuid => "uuid::Uuid".to_string(),
            DataType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
            DataType::Date => "chrono::NaiveDate".to_string(),
            DataType::Time => "chrono::NaiveTime".to_string(),
            DataType::Bytes => "Vec<u8>".to_string(),
            DataType::Json => "serde_json::Value".to_string(),
            DataType::Optional(inner) => format!("Option<{}>", inner.to_rust_type()),
            DataType::Array(inner) => format!("Vec<{}>", inner.to_rust_type()),
            DataType::Map { key, value } => {
                format!(
                    "std::collections::HashMap<{}, {}>",
                    key.to_rust_type(),
                    value.to_rust_type()
                )
            }
            DataType::Reference(entity) | DataType::Entity(entity) => entity.clone(),
            DataType::Any => "Box<dyn std::any::Any>".to_string(),
            DataType::Trigger => "()".to_string(),
            DataType::Custom { type_name, .. } => type_name.clone(),
        }
    }
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

/// Configuration values for component settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<ConfigValue>),
    Object(std::collections::HashMap<String, ConfigValue>),
}

impl ConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ConfigValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            ConfigValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            ConfigValue::Float(v) => Some(*v),
            ConfigValue::Int(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            ConfigValue::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<ConfigValue>> {
        match self {
            ConfigValue::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&std::collections::HashMap<String, ConfigValue>> {
        match self {
            ConfigValue::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, ConfigValue::Null)
    }
}

impl Default for ConfigValue {
    fn default() -> Self {
        ConfigValue::Null
    }
}

impl From<bool> for ConfigValue {
    fn from(v: bool) -> Self {
        ConfigValue::Bool(v)
    }
}

impl From<i64> for ConfigValue {
    fn from(v: i64) -> Self {
        ConfigValue::Int(v)
    }
}

impl From<i32> for ConfigValue {
    fn from(v: i32) -> Self {
        ConfigValue::Int(v as i64)
    }
}

impl From<f64> for ConfigValue {
    fn from(v: f64) -> Self {
        ConfigValue::Float(v)
    }
}

impl From<String> for ConfigValue {
    fn from(v: String) -> Self {
        ConfigValue::String(v)
    }
}

impl From<&str> for ConfigValue {
    fn from(v: &str) -> Self {
        ConfigValue::String(v.to_string())
    }
}

/// Component categories for organizing in the palette
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentCategory {
    /// Authentication components (Login, Register, OAuth)
    Auth,
    /// Data modeling components (Entity, Collection)
    Data,
    /// API components (REST, GraphQL, WebSocket)
    Api,
    /// Storage components (Database, Cache, FileStore)
    Storage,
    /// UI components (Form, Table, Page)
    Ui,
    /// Logic components (Validator, Transformer, Condition)
    Logic,
    /// Embedded/hardware components (GPIO, Sensor, I2C)
    Embedded,
    /// Custom/user-defined components
    Custom,
}

impl ComponentCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentCategory::Auth => "Authentication",
            ComponentCategory::Data => "Data",
            ComponentCategory::Api => "API",
            ComponentCategory::Storage => "Storage",
            ComponentCategory::Ui => "UI",
            ComponentCategory::Logic => "Logic",
            ComponentCategory::Embedded => "Embedded",
            ComponentCategory::Custom => "Custom",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ComponentCategory::Auth => "🔐",
            ComponentCategory::Data => "📊",
            ComponentCategory::Api => "🔌",
            ComponentCategory::Storage => "💾",
            ComponentCategory::Ui => "🖼",
            ComponentCategory::Logic => "⚙",
            ComponentCategory::Embedded => "🔧",
            ComponentCategory::Custom => "📦",
        }
    }

    pub fn all() -> &'static [ComponentCategory] {
        &[
            ComponentCategory::Auth,
            ComponentCategory::Data,
            ComponentCategory::Api,
            ComponentCategory::Storage,
            ComponentCategory::Ui,
            ComponentCategory::Logic,
            ComponentCategory::Embedded,
            ComponentCategory::Custom,
        ]
    }
}

/// Type of connection between components
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionType {
    /// Data flows from source to target
    DataFlow,
    /// UI navigation from source to target
    Navigation,
    /// Entity relationship (one-to-one, one-to-many, etc.)
    Relationship(RelationType),
    /// Event triggers action
    Trigger,
    /// Structural dependency (target requires source)
    Dependency,
}

/// Entity relationship types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl RelationType {
    pub fn display_name(&self) -> &'static str {
        match self {
            RelationType::OneToOne => "One to One",
            RelationType::OneToMany => "One to Many",
            RelationType::ManyToOne => "Many to One",
            RelationType::ManyToMany => "Many to Many",
        }
    }

    pub fn arrow_symbol(&self) -> &'static str {
        match self {
            RelationType::OneToOne => "1 ─── 1",
            RelationType::OneToMany => "1 ───< *",
            RelationType::ManyToOne => "* >─── 1",
            RelationType::ManyToMany => "* >──< *",
        }
    }
}

/// Port types for component connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortDirection {
    /// Input port - receives data/signals
    Input,
    /// Output port - sends data/signals
    Output,
}

/// The kind of port (what it carries)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortKind {
    /// Carries data
    Data,
    /// Carries trigger/event signals
    Trigger,
    /// Control flow
    Flow,
}

/// Validation rules for fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Validation {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Email,
    Url,
    Uuid,
    Custom { name: String, message: String },
}

impl Validation {
    pub fn error_message(&self) -> String {
        match self {
            Validation::Required => "This field is required".to_string(),
            Validation::MinLength(n) => format!("Minimum length is {}", n),
            Validation::MaxLength(n) => format!("Maximum length is {}", n),
            Validation::Min(n) => format!("Minimum value is {}", n),
            Validation::Max(n) => format!("Maximum value is {}", n),
            Validation::Pattern(p) => format!("Must match pattern: {}", p),
            Validation::Email => "Must be a valid email address".to_string(),
            Validation::Url => "Must be a valid URL".to_string(),
            Validation::Uuid => "Must be a valid UUID".to_string(),
            Validation::Custom { message, .. } => message.clone(),
        }
    }
}

/// UI hints for rendering fields in the editor
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UiHints {
    /// Display label (defaults to field name if not set)
    pub label: Option<String>,
    /// Placeholder text for input
    pub placeholder: Option<String>,
    /// Help text / description
    pub help: Option<String>,
    /// Whether this is a secret field (passwords, API keys)
    pub secret: bool,
    /// Whether this field should be shown in list views
    pub show_in_list: bool,
    /// Custom widget type
    pub widget: Option<String>,
    /// Display order (lower = earlier)
    pub order: i32,
}

impl UiHints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn secret(mut self) -> Self {
        self.secret = true;
        self
    }

    pub fn show_in_list(mut self) -> Self {
        self.show_in_list = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(Position::new(10.0, 10.0), Size::new(100.0, 50.0));
        assert!(rect.contains(Position::new(50.0, 30.0)));
        assert!(!rect.contains(Position::new(5.0, 30.0)));
    }

    #[test]
    fn test_data_type_compatibility() {
        assert!(DataType::String.is_compatible_with(&DataType::String));
        assert!(DataType::Any.is_compatible_with(&DataType::Int32));
        assert!(DataType::Int32.is_compatible_with(&DataType::Int64));
        assert!(!DataType::String.is_compatible_with(&DataType::Int32));
    }

    #[test]
    fn test_config_value_conversions() {
        let val: ConfigValue = "test".into();
        assert_eq!(val.as_str(), Some("test"));

        let val: ConfigValue = 42i32.into();
        assert_eq!(val.as_int(), Some(42));
    }
}
