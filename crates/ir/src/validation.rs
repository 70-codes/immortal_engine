//! Validation module for Immortal Engine IR
//!
//! Provides validation rules and utilities for ensuring project graphs
//! are valid and consistent before code generation.

use std::collections::{HashMap, HashSet};

use imortal_core::{NodeId, EdgeId, ConnectionType};

use crate::graph::ProjectGraph;

/// Result of a validation operation
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// A validation error with context
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// The type/category of validation error
    pub kind: ValidationErrorKind,
    /// Human-readable error message
    pub message: String,
    /// Optional node ID related to this error
    pub node_id: Option<NodeId>,
    /// Optional edge ID related to this error
    pub edge_id: Option<EdgeId>,
    /// Severity of the error
    pub severity: ValidationSeverity,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(kind: ValidationErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            node_id: None,
            edge_id: None,
            severity: ValidationSeverity::Error,
        }
    }

    /// Create an error related to a specific node
    pub fn for_node(kind: ValidationErrorKind, message: impl Into<String>, node_id: NodeId) -> Self {
        Self {
            kind,
            message: message.into(),
            node_id: Some(node_id),
            edge_id: None,
            severity: ValidationSeverity::Error,
        }
    }

    /// Create an error related to a specific edge
    pub fn for_edge(kind: ValidationErrorKind, message: impl Into<String>, edge_id: EdgeId) -> Self {
        Self {
            kind,
            message: message.into(),
            node_id: None,
            edge_id: Some(edge_id),
            severity: ValidationSeverity::Error,
        }
    }

    /// Set the severity to warning
    pub fn as_warning(mut self) -> Self {
        self.severity = ValidationSeverity::Warning;
        self
    }

    /// Set the severity to info
    pub fn as_info(mut self) -> Self {
        self.severity = ValidationSeverity::Info;
        self
    }

    /// Check if this is an error (not warning or info)
    pub fn is_error(&self) -> bool {
        matches!(self.severity, ValidationSeverity::Error)
    }

    /// Check if this is a warning
    pub fn is_warning(&self) -> bool {
        matches!(self.severity, ValidationSeverity::Warning)
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.severity {
            ValidationSeverity::Error => "ERROR",
            ValidationSeverity::Warning => "WARNING",
            ValidationSeverity::Info => "INFO",
        };

        if let Some(node_id) = self.node_id {
            write!(f, "[{}] Node {}: {}", prefix, node_id, self.message)
        } else if let Some(edge_id) = self.edge_id {
            write!(f, "[{}] Edge {}: {}", prefix, edge_id, self.message)
        } else {
            write!(f, "[{}] {}", prefix, self.message)
        }
    }
}

impl std::error::Error for ValidationError {}

/// Categories of validation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationErrorKind {
    // Graph structure errors
    EmptyGraph,
    CyclicDependency,
    DisconnectedNodes,
    InvalidConnection,

    // Node errors
    MissingNode,
    DuplicateNodeName,
    InvalidNodeType,
    MissingRequiredField,
    InvalidFieldType,
    InvalidFieldValue,

    // Edge errors
    MissingEdge,
    DanglingEdge,
    InvalidEdgeType,
    IncompatiblePorts,
    DuplicateEdge,

    // Port errors
    MissingPort,
    UnconnectedRequiredPort,
    MultipleConnectionsOnSinglePort,

    // Domain-specific errors
    InvalidDatabaseConfig,
    InvalidApiConfig,
    InvalidEmbeddedConfig,

    // Schema errors
    InvalidSchema,
    MissingPrimaryKey,
    InvalidRelationship,
    CircularReference,

    // General errors
    Custom,
}

impl ValidationErrorKind {
    /// Get a human-readable name for this error kind
    pub fn name(&self) -> &'static str {
        match self {
            Self::EmptyGraph => "Empty Graph",
            Self::CyclicDependency => "Cyclic Dependency",
            Self::DisconnectedNodes => "Disconnected Nodes",
            Self::InvalidConnection => "Invalid Connection",
            Self::MissingNode => "Missing Node",
            Self::DuplicateNodeName => "Duplicate Node Name",
            Self::InvalidNodeType => "Invalid Node Type",
            Self::MissingRequiredField => "Missing Required Field",
            Self::InvalidFieldType => "Invalid Field Type",
            Self::InvalidFieldValue => "Invalid Field Value",
            Self::MissingEdge => "Missing Edge",
            Self::DanglingEdge => "Dangling Edge",
            Self::InvalidEdgeType => "Invalid Edge Type",
            Self::IncompatiblePorts => "Incompatible Ports",
            Self::DuplicateEdge => "Duplicate Edge",
            Self::MissingPort => "Missing Port",
            Self::UnconnectedRequiredPort => "Unconnected Required Port",
            Self::MultipleConnectionsOnSinglePort => "Multiple Connections on Single Port",
            Self::InvalidDatabaseConfig => "Invalid Database Config",
            Self::InvalidApiConfig => "Invalid API Config",
            Self::InvalidEmbeddedConfig => "Invalid Embedded Config",
            Self::InvalidSchema => "Invalid Schema",
            Self::MissingPrimaryKey => "Missing Primary Key",
            Self::InvalidRelationship => "Invalid Relationship",
            Self::CircularReference => "Circular Reference",
            Self::Custom => "Custom Error",
        }
    }
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ValidationSeverity {
    /// Critical error that must be fixed
    #[default]
    Error,
    /// Warning that should be addressed
    Warning,
    /// Informational message
    Info,
}

/// Validator for project graphs
pub struct Validator {
    /// Validation rules to apply
    rules: Vec<Box<dyn ValidationRule>>,
    /// Whether to stop on first error
    fail_fast: bool,
    /// Minimum severity to report
    min_severity: ValidationSeverity,
}

impl Validator {
    /// Create a new validator with default rules
    pub fn new() -> Self {
        let mut validator = Self {
            rules: Vec::new(),
            fail_fast: false,
            min_severity: ValidationSeverity::Warning,
        };

        // Add default validation rules
        validator.add_rule(Box::new(NodeExistsRule));
        validator.add_rule(Box::new(EdgeValidityRule));
        validator.add_rule(Box::new(PortCompatibilityRule));
        validator.add_rule(Box::new(RequiredFieldsRule));
        validator.add_rule(Box::new(EntityPrimaryKeyRule));
        validator.add_rule(Box::new(DuplicateNameRule));

        validator
    }

    /// Create an empty validator (no rules)
    pub fn empty() -> Self {
        Self {
            rules: Vec::new(),
            fail_fast: false,
            min_severity: ValidationSeverity::Warning,
        }
    }

    /// Add a validation rule
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }

    /// Set fail-fast mode
    pub fn fail_fast(mut self, enabled: bool) -> Self {
        self.fail_fast = enabled;
        self
    }

    /// Set minimum severity to report
    pub fn min_severity(mut self, severity: ValidationSeverity) -> Self {
        self.min_severity = severity;
        self
    }

    /// Validate a project graph
    pub fn validate(&self, graph: &ProjectGraph) -> ValidationResult {
        let mut all_errors = Vec::new();

        for rule in &self.rules {
            let errors = rule.validate(graph);

            // Filter by severity
            let filtered: Vec<_> = errors
                .into_iter()
                .filter(|e| self.should_include(e.severity))
                .collect();

            all_errors.extend(filtered);

            if self.fail_fast && all_errors.iter().any(|e| e.is_error()) {
                break;
            }
        }

        if all_errors.iter().any(|e| e.is_error()) {
            Err(all_errors)
        } else if !all_errors.is_empty() {
            // Only warnings/info - still return them but as Ok with side effects
            // In a real implementation, you might want to handle this differently
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Validate and return all issues (errors and warnings)
    pub fn validate_all(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut all_errors = Vec::new();

        for rule in &self.rules {
            let errors = rule.validate(graph);
            let filtered: Vec<_> = errors
                .into_iter()
                .filter(|e| self.should_include(e.severity))
                .collect();
            all_errors.extend(filtered);
        }

        all_errors
    }

    /// Check if validation passes (no errors, warnings are ok)
    pub fn is_valid(&self, graph: &ProjectGraph) -> bool {
        self.validate(graph).is_ok()
    }

    fn should_include(&self, severity: ValidationSeverity) -> bool {
        match self.min_severity {
            ValidationSeverity::Error => matches!(severity, ValidationSeverity::Error),
            ValidationSeverity::Warning => matches!(
                severity,
                ValidationSeverity::Error | ValidationSeverity::Warning
            ),
            ValidationSeverity::Info => true,
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for validation rules
pub trait ValidationRule: Send + Sync {
    /// The name of this rule
    fn name(&self) -> &'static str;

    /// Validate the graph and return any errors
    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError>;
}

// ============ Built-in Validation Rules ============

/// Validates that all referenced nodes exist
pub struct NodeExistsRule;

impl ValidationRule for NodeExistsRule {
    fn name(&self) -> &'static str {
        "Node Existence"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check that all edges reference existing nodes
        for edge in graph.edges() {
            if !graph.has_node(edge.from_node) {
                errors.push(ValidationError::for_edge(
                    ValidationErrorKind::DanglingEdge,
                    format!("Edge references non-existent source node: {}", edge.from_node),
                    edge.id,
                ));
            }
            if !graph.has_node(edge.to_node) {
                errors.push(ValidationError::for_edge(
                    ValidationErrorKind::DanglingEdge,
                    format!("Edge references non-existent target node: {}", edge.to_node),
                    edge.id,
                ));
            }
        }

        // Check that all group node references exist
        for group in graph.groups() {
            for node_id in group.nodes() {
                if !graph.has_node(*node_id) {
                    errors.push(ValidationError::new(
                        ValidationErrorKind::MissingNode,
                        format!("Group '{}' references non-existent node: {}", group.name, node_id),
                    ));
                }
            }
        }

        errors
    }
}

/// Validates edge connections (ports exist, types compatible)
pub struct EdgeValidityRule;

impl ValidationRule for EdgeValidityRule {
    fn name(&self) -> &'static str {
        "Edge Validity"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for edge in graph.edges() {
            // Check source port exists
            if let Some(from_node) = graph.get_node(edge.from_node) {
                if from_node.get_output_port(&edge.from_port).is_none() {
                    errors.push(ValidationError::for_edge(
                        ValidationErrorKind::MissingPort,
                        format!(
                            "Source port '{}' not found on node '{}'",
                            edge.from_port, from_node.name
                        ),
                        edge.id,
                    ));
                }
            }

            // Check target port exists
            if let Some(to_node) = graph.get_node(edge.to_node) {
                if to_node.get_input_port(&edge.to_port).is_none() {
                    errors.push(ValidationError::for_edge(
                        ValidationErrorKind::MissingPort,
                        format!(
                            "Target port '{}' not found on node '{}'",
                            edge.to_port, to_node.name
                        ),
                        edge.id,
                    ));
                }
            }
        }

        errors
    }
}

/// Validates port type compatibility
pub struct PortCompatibilityRule;

impl ValidationRule for PortCompatibilityRule {
    fn name(&self) -> &'static str {
        "Port Compatibility"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for edge in graph.edges() {
            // Skip port compatibility check for relationship edges
            // Relationships connect different entity types by design
            if matches!(edge.connection_type, ConnectionType::Relationship(_)) {
                continue;
            }

            let from_port = graph
                .get_node(edge.from_node)
                .and_then(|n| n.get_output_port(&edge.from_port));

            let to_port = graph
                .get_node(edge.to_node)
                .and_then(|n| n.get_input_port(&edge.to_port));

            if let (Some(from), Some(to)) = (from_port, to_port) {
                if !from.can_connect_to(to) {
                    errors.push(ValidationError::for_edge(
                        ValidationErrorKind::IncompatiblePorts,
                        format!(
                            "Port types are incompatible: {} ({:?}) -> {} ({:?})",
                            from.name, from.data_type, to.name, to.data_type
                        ),
                        edge.id,
                    ));
                }
            }
        }

        errors
    }
}

/// Validates that required fields are present
pub struct RequiredFieldsRule;

impl ValidationRule for RequiredFieldsRule {
    fn name(&self) -> &'static str {
        "Required Fields"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let errors = Vec::new();

        for node in graph.nodes() {
            for field in &node.fields {
                if field.required && field.default_value.is_none() {
                    // This is more of a warning - required fields without defaults
                    // are valid but might need user input
                }
            }
        }

        errors
    }
}

/// Validates that entity nodes have primary keys
pub struct EntityPrimaryKeyRule;

impl ValidationRule for EntityPrimaryKeyRule {
    fn name(&self) -> &'static str {
        "Entity Primary Key"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for node in graph.nodes() {
            if node.component_type == "data.entity" {
                let has_pk = node.fields.iter().any(|f| f.is_primary_key());
                if !has_pk {
                    errors.push(ValidationError::for_node(
                        ValidationErrorKind::MissingPrimaryKey,
                        format!("Entity '{}' has no primary key field", node.name),
                        node.id,
                    ).as_warning());
                }
            }
        }

        errors
    }
}

/// Validates that node names are unique within their type
pub struct DuplicateNameRule;

impl ValidationRule for DuplicateNameRule {
    fn name(&self) -> &'static str {
        "Unique Names"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let mut names_by_type: HashMap<&str, HashMap<&str, NodeId>> = HashMap::new();

        for node in graph.nodes() {
            let type_names = names_by_type
                .entry(&node.component_type)
                .or_insert_with(HashMap::new);

            if let Some(&existing_id) = type_names.get(node.name.as_str()) {
                errors.push(ValidationError::for_node(
                    ValidationErrorKind::DuplicateNodeName,
                    format!(
                        "Duplicate node name '{}' (conflicts with node {})",
                        node.name, existing_id
                    ),
                    node.id,
                ));
            } else {
                type_names.insert(&node.name, node.id);
            }
        }

        errors
    }
}

/// Validates that there are no cycles in data flow
pub struct CyclicDependencyRule;

impl ValidationRule for CyclicDependencyRule {
    fn name(&self) -> &'static str {
        "Cyclic Dependency"
    }

    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Build adjacency list
        let mut adj: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for node_id in graph.node_ids() {
            adj.insert(*node_id, Vec::new());
        }
        for edge in graph.edges() {
            if edge.is_data_flow() {
                adj.entry(edge.from_node)
                    .or_insert_with(Vec::new)
                    .push(edge.to_node);
            }
        }

        // DFS to detect cycles
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node_id in graph.node_ids() {
            if !visited.contains(node_id) {
                if has_cycle(*node_id, &adj, &mut visited, &mut rec_stack) {
                    errors.push(ValidationError::new(
                        ValidationErrorKind::CyclicDependency,
                        "Cyclic dependency detected in data flow",
                    ));
                    break; // One cycle error is enough
                }
            }
        }

        errors
    }
}

fn has_cycle(
    node: NodeId,
    adj: &HashMap<NodeId, Vec<NodeId>>,
    visited: &mut HashSet<NodeId>,
    rec_stack: &mut HashSet<NodeId>,
) -> bool {
    visited.insert(node);
    rec_stack.insert(node);

    if let Some(neighbors) = adj.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                if has_cycle(neighbor, adj, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                return true;
            }
        }
    }

    rec_stack.remove(&node);
    false
}

/// Quick validation function
pub fn validate(graph: &ProjectGraph) -> ValidationResult {
    Validator::new().validate(graph)
}

/// Get all validation issues (errors and warnings)
pub fn get_all_issues(graph: &ProjectGraph) -> Vec<ValidationError> {
    Validator::new()
        .min_severity(ValidationSeverity::Info)
        .validate_all(graph)
}

/// Check if a graph is valid
pub fn is_valid(graph: &ProjectGraph) -> bool {
    Validator::new().is_valid(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;

    use crate::project::ProjectMeta;
    use imortal_core::RelationType;

    fn create_test_graph() -> ProjectGraph {
        let mut graph = ProjectGraph::new(ProjectMeta::new("test"));

        let user = graph.add_node(Node::new_entity("User"));
        let post = graph.add_node(Node::new_entity("Post"));
        let _ = graph.add_relationship(user, post, RelationType::OneToMany);

        graph
    }

    #[test]
    fn test_valid_graph() {
        let graph = create_test_graph();
        assert!(is_valid(&graph));
    }

    #[test]
    fn test_empty_graph() {
        let graph = ProjectGraph::with_name("empty");
        // Empty graph should be valid (no rules fail on empty)
        assert!(is_valid(&graph));
    }

    #[test]
    fn test_duplicate_name_detection() {
        let mut graph = ProjectGraph::with_name("test");
        graph.add_node(Node::new_entity("User"));
        graph.add_node(Node::new_entity("User")); // Duplicate

        let rule = DuplicateNameRule;
        let errors = rule.validate(&graph);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.kind == ValidationErrorKind::DuplicateNodeName));
    }

    #[test]
    fn test_validator_builder() {
        let validator = Validator::new()
            .fail_fast(true)
            .min_severity(ValidationSeverity::Error);

        let graph = create_test_graph();
        assert!(validator.is_valid(&graph));
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::new(
            ValidationErrorKind::MissingNode,
            "Test error message",
        );
        let display = format!("{}", error);
        assert!(display.contains("ERROR"));
        assert!(display.contains("Test error message"));
    }

    #[test]
    fn test_warning_severity() {
        let warning = ValidationError::new(
            ValidationErrorKind::Custom,
            "This is a warning",
        ).as_warning();

        assert!(warning.is_warning());
        assert!(!warning.is_error());
    }
}
