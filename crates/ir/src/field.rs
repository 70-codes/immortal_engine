//! Field types for component fields
//!
//! Fields represent the data members within components. For example, a Login component
//! might have `email` and `password` fields, while a User entity might have `id`,
//! `username`, and `created_at` fields.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use imortal_core::{DataType, UiHints, Validation, ConfigValue};

/// A field within a component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// Unique identifier for this field
    pub id: Uuid,

    /// Field name (used in code generation)
    pub name: String,

    /// Display label (shown in UI, defaults to name if not set)
    pub label: Option<String>,

    /// Data type of the field
    pub data_type: DataType,

    /// Whether this field is required
    pub required: bool,

    /// Default value for the field
    pub default_value: Option<ConfigValue>,

    /// Validation rules applied to this field
    pub validations: Vec<Validation>,

    /// UI hints for rendering in the editor
    pub ui_hints: UiHints,

    /// Field constraints (database-level)
    pub constraints: Vec<FieldConstraint>,

    /// Optional description/documentation
    pub description: Option<String>,

    /// Whether this field is read-only
    pub read_only: bool,

    /// Whether this field is deprecated
    pub deprecated: bool,

    /// Deprecation message if deprecated
    pub deprecation_message: Option<String>,

    /// Custom metadata
    pub metadata: std::collections::HashMap<String, ConfigValue>,
}

impl Field {
    /// Create a new field with the given name and data type
    pub fn new(name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            label: None,
            data_type,
            required: false,
            default_value: None,
            validations: Vec::new(),
            ui_hints: UiHints::default(),
            constraints: Vec::new(),
            description: None,
            read_only: false,
            deprecated: false,
            deprecation_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a new string field
    pub fn string(name: impl Into<String>) -> Self {
        Self::new(name, DataType::String)
    }

    /// Create a new text field (for long content)
    pub fn text(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Text)
    }

    /// Create a new integer field (i32)
    pub fn int(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Int32)
    }

    /// Create a new long integer field (i64)
    pub fn long(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Int64)
    }

    /// Create a new float field (f32)
    pub fn float(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Float32)
    }

    /// Create a new double field (f64)
    pub fn double(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Float64)
    }

    /// Create a new boolean field
    pub fn bool(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Bool)
    }

    /// Create a new UUID field
    pub fn uuid(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Uuid)
    }

    /// Create a new datetime field
    pub fn datetime(name: impl Into<String>) -> Self {
        Self::new(name, DataType::DateTime)
    }

    /// Create a new date field
    pub fn date(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Date)
    }

    /// Create a new bytes/binary field
    pub fn bytes(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Bytes)
    }

    /// Create a new JSON field
    pub fn json(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Json)
    }

    /// Create a reference field pointing to another entity
    pub fn reference(name: impl Into<String>, target_entity: impl Into<String>) -> Self {
        Self::new(name, DataType::Reference(target_entity.into()))
    }

    /// Create an array/list field
    pub fn array(name: impl Into<String>, element_type: DataType) -> Self {
        Self::new(name, DataType::Array(Box::new(element_type)))
    }

    /// Create an optional field
    pub fn optional(name: impl Into<String>, inner_type: DataType) -> Self {
        Self::new(name, DataType::Optional(Box::new(inner_type)))
    }

    // ============ Builder Methods ============

    /// Set the display label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Mark this field as required
    pub fn required(mut self) -> Self {
        self.required = true;
        if !self.validations.iter().any(|v| matches!(v, Validation::Required)) {
            self.validations.push(Validation::Required);
        }
        self
    }

    /// Set a default value
    pub fn with_default(mut self, value: impl Into<ConfigValue>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Add a validation rule
    pub fn with_validation(mut self, validation: Validation) -> Self {
        self.validations.push(validation);
        self
    }

    /// Add a constraint
    pub fn with_constraint(mut self, constraint: FieldConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Mark as primary key
    pub fn primary_key(mut self) -> Self {
        self.constraints.push(FieldConstraint::PrimaryKey);
        self.required = true;
        self
    }

    /// Mark as unique
    pub fn unique(mut self) -> Self {
        self.constraints.push(FieldConstraint::Unique);
        self
    }

    /// Add an index
    pub fn indexed(mut self) -> Self {
        self.constraints.push(FieldConstraint::Indexed);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Mark as read-only
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Mark as secret (passwords, API keys, etc.)
    pub fn secret(mut self) -> Self {
        self.ui_hints.secret = true;
        self
    }

    /// Set the placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.ui_hints.placeholder = Some(placeholder.into());
        self
    }

    /// Set the help text
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.ui_hints.help = Some(help.into());
        self
    }

    /// Show in list views
    pub fn show_in_list(mut self) -> Self {
        self.ui_hints.show_in_list = true;
        self
    }

    /// Set custom UI widget
    pub fn with_widget(mut self, widget: impl Into<String>) -> Self {
        self.ui_hints.widget = Some(widget.into());
        self
    }

    /// Set display order
    pub fn with_order(mut self, order: i32) -> Self {
        self.ui_hints.order = order;
        self
    }

    /// Mark as deprecated
    pub fn deprecated(mut self, message: impl Into<String>) -> Self {
        self.deprecated = true;
        self.deprecation_message = Some(message.into());
        self
    }

    /// Add custom metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<ConfigValue>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    // ============ Utility Methods ============

    /// Get the display label (falls back to name)
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.name)
    }

    /// Check if this field has a specific constraint
    pub fn has_constraint(&self, constraint: &FieldConstraint) -> bool {
        self.constraints.contains(constraint)
    }

    /// Check if this is a primary key field
    pub fn is_primary_key(&self) -> bool {
        self.has_constraint(&FieldConstraint::PrimaryKey)
    }

    /// Check if this field is unique
    pub fn is_unique(&self) -> bool {
        self.has_constraint(&FieldConstraint::Unique) || self.is_primary_key()
    }

    /// Check if this is a foreign key field
    pub fn is_foreign_key(&self) -> bool {
        self.constraints.iter().any(|c| matches!(c, FieldConstraint::ForeignKey { .. }))
    }

    /// Get the Rust type for this field
    pub fn rust_type(&self) -> String {
        let base_type = self.data_type.to_rust_type();
        if self.required || self.is_primary_key() {
            base_type
        } else {
            format!("Option<{}>", base_type)
        }
    }

    /// Validate a value against this field's validations
    pub fn validate_value(&self, value: &ConfigValue) -> Vec<String> {
        let mut errors = Vec::new();

        // Check required
        if self.required && value.is_null() {
            errors.push(format!("Field '{}' is required", self.name));
        }

        // Check validations
        for validation in &self.validations {
            if let Some(error) = self.check_validation(validation, value) {
                errors.push(error);
            }
        }

        errors
    }

    fn check_validation(&self, validation: &Validation, value: &ConfigValue) -> Option<String> {
        match validation {
            Validation::Required if value.is_null() => {
                Some(format!("'{}' is required", self.name))
            }
            Validation::MinLength(min) => {
                if let Some(s) = value.as_str() {
                    if s.len() < *min {
                        return Some(format!("'{}' must be at least {} characters", self.name, min));
                    }
                }
                None
            }
            Validation::MaxLength(max) => {
                if let Some(s) = value.as_str() {
                    if s.len() > *max {
                        return Some(format!("'{}' must be at most {} characters", self.name, max));
                    }
                }
                None
            }
            Validation::Min(min) => {
                if let Some(n) = value.as_float() {
                    if n < *min {
                        return Some(format!("'{}' must be at least {}", self.name, min));
                    }
                }
                None
            }
            Validation::Max(max) => {
                if let Some(n) = value.as_float() {
                    if n > *max {
                        return Some(format!("'{}' must be at most {}", self.name, max));
                    }
                }
                None
            }
            Validation::Email => {
                if let Some(s) = value.as_str() {
                    if !s.contains('@') || !s.contains('.') {
                        return Some(format!("'{}' must be a valid email address", self.name));
                    }
                }
                None
            }
            _ => None,
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::string("field")
    }
}

/// Field constraints (database-level)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldConstraint {
    /// Primary key constraint
    PrimaryKey,

    /// Unique constraint
    Unique,

    /// Indexed for faster queries
    Indexed,

    /// Foreign key reference
    ForeignKey {
        /// Target entity name
        entity: String,
        /// Target field name
        field: String,
        /// Action on delete
        on_delete: ForeignKeyAction,
        /// Action on update
        on_update: ForeignKeyAction,
    },

    /// Auto-increment (for integer primary keys)
    AutoIncrement,

    /// Check constraint with expression
    Check(String),

    /// Default value expression (database-level)
    DefaultExpression(String),
}

/// Foreign key actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ForeignKeyAction {
    /// Do nothing (may cause errors)
    #[default]
    NoAction,

    /// Restrict deletion/update
    Restrict,

    /// Cascade the operation
    Cascade,

    /// Set to NULL
    SetNull,

    /// Set to default value
    SetDefault,
}

impl ForeignKeyAction {
    pub fn to_sql(&self) -> &'static str {
        match self {
            ForeignKeyAction::NoAction => "NO ACTION",
            ForeignKeyAction::Restrict => "RESTRICT",
            ForeignKeyAction::Cascade => "CASCADE",
            ForeignKeyAction::SetNull => "SET NULL",
            ForeignKeyAction::SetDefault => "SET DEFAULT",
        }
    }
}

/// Builder for creating foreign key constraints
pub struct ForeignKeyBuilder {
    entity: String,
    field: String,
    on_delete: ForeignKeyAction,
    on_update: ForeignKeyAction,
}

impl ForeignKeyBuilder {
    pub fn new(entity: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            entity: entity.into(),
            field: field.into(),
            on_delete: ForeignKeyAction::NoAction,
            on_update: ForeignKeyAction::NoAction,
        }
    }

    pub fn on_delete(mut self, action: ForeignKeyAction) -> Self {
        self.on_delete = action;
        self
    }

    pub fn on_update(mut self, action: ForeignKeyAction) -> Self {
        self.on_update = action;
        self
    }

    pub fn cascade(self) -> Self {
        self.on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
    }

    pub fn build(self) -> FieldConstraint {
        FieldConstraint::ForeignKey {
            entity: self.entity,
            field: self.field,
            on_delete: self.on_delete,
            on_update: self.on_update,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::string("username")
            .required()
            .with_label("Username")
            .with_placeholder("Enter your username")
            .unique();

        assert_eq!(field.name, "username");
        assert_eq!(field.display_label(), "Username");
        assert!(field.required);
        assert!(field.is_unique());
    }

    #[test]
    fn test_primary_key_field() {
        let field = Field::uuid("id").primary_key();

        assert!(field.is_primary_key());
        assert!(field.is_unique());
        assert!(field.required);
    }

    #[test]
    fn test_foreign_key_builder() {
        let fk = ForeignKeyBuilder::new("User", "id")
            .cascade()
            .build();

        match fk {
            FieldConstraint::ForeignKey { entity, field, on_delete, on_update } => {
                assert_eq!(entity, "User");
                assert_eq!(field, "id");
                assert_eq!(on_delete, ForeignKeyAction::Cascade);
                assert_eq!(on_update, ForeignKeyAction::Cascade);
            }
            _ => panic!("Expected ForeignKey constraint"),
        }
    }

    #[test]
    fn test_field_rust_type() {
        let required = Field::string("name").required();
        let optional = Field::string("nickname");

        assert_eq!(required.rust_type(), "String");
        assert_eq!(optional.rust_type(), "Option<String>");
    }

    #[test]
    fn test_secret_field() {
        let password = Field::string("password")
            .required()
            .secret()
            .with_validation(Validation::MinLength(8));

        assert!(password.ui_hints.secret);
        assert!(password.validations.iter().any(|v| matches!(v, Validation::MinLength(8))));
    }
}
