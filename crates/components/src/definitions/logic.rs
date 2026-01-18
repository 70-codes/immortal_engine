//! Logic Component Definitions
//!
//! This module provides component definitions for logic and flow control:
//! - Validator: Validate data against rules
//! - Transformer: Transform/map data between formats
//! - Condition: Conditional branching based on expressions

use crate::definition::{
    ComponentDefinition, ConfigOption, ConfigType, FieldDefinition, PortDefinition,
};
use imortal_core::{ComponentCategory, DataType, Validation};

/// Create the Validator component definition
///
/// A Validator component checks data against a set of validation rules
/// and outputs whether the data is valid along with any error messages.
pub fn validator_component() -> ComponentDefinition {
    ComponentDefinition::new("logic.validator", "Validator", ComponentCategory::Logic)
        .with_description("Validate data against configurable rules")
        .with_icon("✅")
        .with_tag("validation")
        .with_tag("rules")
        .with_tag("check")
        // Input ports
        .with_input(
            PortDefinition::data_in("data", "Data", DataType::Any)
                .with_description("Data to validate")
                .required(),
        )
        .with_input(
            PortDefinition::trigger_in("validate", "Validate")
                .with_description("Trigger validation"),
        )
        .with_input(
            PortDefinition::data_in("rules", "Rules", DataType::Json)
                .with_description("Dynamic validation rules (overrides configured rules)"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("data", "Valid Data", DataType::Any)
                .with_description("The validated data (same as input if valid)"),
        )
        .with_output(
            PortDefinition::data_out("is_valid", "Is Valid", DataType::Bool)
                .with_description("Whether the data passed validation"),
        )
        .with_output(
            PortDefinition::data_out("errors", "Errors", DataType::Array(Box::new(DataType::String)))
                .with_description("List of validation error messages"),
        )
        .with_output(
            PortDefinition::data_out("field_errors", "Field Errors", DataType::Json)
                .with_description("Field-specific validation errors"),
        )
        .with_output(
            PortDefinition::trigger_out("valid", "On Valid")
                .with_description("Triggered when validation passes"),
        )
        .with_output(
            PortDefinition::trigger_out("invalid", "On Invalid")
                .with_description("Triggered when validation fails"),
        )
        // Fields for defining validation rules
        .with_field(
            FieldDefinition::text("rules_json")
                .with_label("Validation Rules (JSON)")
                .with_placeholder(r#"{"email": ["required", "email"], "age": ["min:18"]}"#)
                .with_description("JSON object mapping field names to validation rules")
                .with_widget("code"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("fail_fast", "Fail Fast")
                .with_description("Stop on first validation error")
                .with_default(false),
        )
        .with_config(
            ConfigOption::boolean("trim_strings", "Trim Strings")
                .with_description("Trim whitespace from string values before validation")
                .with_default(true),
        )
        .with_config(
            ConfigOption::boolean("allow_unknown_fields", "Allow Unknown Fields")
                .with_description("Allow fields not defined in the rules")
                .with_default(true),
        )
        .with_config(
            ConfigOption::select("error_format", "Error Format")
                .with_option("simple", "Simple (list of strings)")
                .with_option("detailed", "Detailed (field -> errors)")
                .with_option("both", "Both")
                .with_default("both"),
        )
        .with_default_size(200.0, 160.0)
        .with_generator("logic::validator")
}

/// Create the Transformer component definition
///
/// A Transformer component maps/transforms data from one format to another,
/// with support for field mapping, type conversion, and custom expressions.
pub fn transformer_component() -> ComponentDefinition {
    ComponentDefinition::new("logic.transformer", "Transformer", ComponentCategory::Logic)
        .with_description("Transform and map data between formats")
        .with_icon("🔄")
        .with_tag("transform")
        .with_tag("map")
        .with_tag("convert")
        // Input ports
        .with_input(
            PortDefinition::data_in("input", "Input", DataType::Any)
                .with_description("Data to transform")
                .required(),
        )
        .with_input(
            PortDefinition::trigger_in("transform", "Transform")
                .with_description("Trigger transformation"),
        )
        .with_input(
            PortDefinition::data_in("context", "Context", DataType::Json)
                .with_description("Additional context data for transformation"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("output", "Output", DataType::Any)
                .with_description("The transformed data"),
        )
        .with_output(
            PortDefinition::trigger_out("success", "On Success")
                .with_description("Triggered when transformation succeeds"),
        )
        .with_output(
            PortDefinition::trigger_out("error", "On Error")
                .with_description("Triggered when transformation fails"),
        )
        .with_output(
            PortDefinition::data_out("error_message", "Error Message", DataType::String)
                .with_description("Error message if transformation fails"),
        )
        // Fields
        .with_field(
            FieldDefinition::text("mapping")
                .with_label("Field Mapping")
                .with_placeholder(r#"{"targetField": "sourceField", "fullName": "first_name + ' ' + last_name"}"#)
                .with_description("JSON mapping from target fields to source fields or expressions")
                .with_widget("code"),
        )
        .with_field(
            FieldDefinition::text("expression")
                .with_label("Transform Expression")
                .with_placeholder("input.map(x => ({ ...x, processed: true }))")
                .with_description("Custom transformation expression (alternative to field mapping)"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("mode", "Transform Mode")
                .with_option("mapping", "Field Mapping")
                .with_option("expression", "Expression")
                .with_option("template", "Template")
                .with_default("mapping")
                .with_description("How to define the transformation"),
        )
        .with_config(
            ConfigOption::boolean("preserve_unmapped", "Preserve Unmapped Fields")
                .with_description("Include fields not in the mapping")
                .with_default(false),
        )
        .with_config(
            ConfigOption::boolean("null_on_missing", "Null on Missing")
                .with_description("Set null for missing source fields instead of error")
                .with_default(true),
        )
        .with_config(
            ConfigOption::select("output_type", "Output Type")
                .with_option("object", "Object")
                .with_option("array", "Array")
                .with_option("string", "String")
                .with_option("infer", "Infer from Input")
                .with_default("infer"),
        )
        .with_config(
            ConfigOption::boolean("deep_clone", "Deep Clone")
                .with_description("Deep clone input before transformation")
                .with_default(true)
                .advanced(),
        )
        .with_default_size(200.0, 180.0)
        .with_generator("logic::transformer")
}

/// Create the Condition component definition
///
/// A Condition component evaluates an expression and routes flow
/// based on whether the condition is true or false.
pub fn condition_component() -> ComponentDefinition {
    ComponentDefinition::new("logic.condition", "Condition", ComponentCategory::Logic)
        .with_description("Conditional branching based on an expression")
        .with_icon("🔀")
        .with_tag("condition")
        .with_tag("if")
        .with_tag("branch")
        .with_tag("flow")
        // Input ports
        .with_input(
            PortDefinition::data_in("value", "Value", DataType::Any)
                .with_description("Value to evaluate")
                .required(),
        )
        .with_input(
            PortDefinition::trigger_in("evaluate", "Evaluate")
                .with_description("Trigger condition evaluation"),
        )
        .with_input(
            PortDefinition::data_in("context", "Context", DataType::Json)
                .with_description("Additional context for expression evaluation"),
        )
        // Output ports
        .with_output(
            PortDefinition::trigger_out("true", "If True")
                .with_description("Triggered when condition is true"),
        )
        .with_output(
            PortDefinition::trigger_out("false", "If False")
                .with_description("Triggered when condition is false"),
        )
        .with_output(
            PortDefinition::data_out("value", "Value", DataType::Any)
                .with_description("Pass-through of the input value"),
        )
        .with_output(
            PortDefinition::data_out("result", "Result", DataType::Bool)
                .with_description("The boolean result of the condition"),
        )
        // Fields
        .with_field(
            FieldDefinition::text("expression")
                .with_label("Condition Expression")
                .required()
                .with_placeholder("value > 10 && value < 100")
                .with_description("Boolean expression to evaluate"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("operator", "Quick Operator")
                .with_option("custom", "Custom Expression")
                .with_option("eq", "Equals (==)")
                .with_option("neq", "Not Equals (!=)")
                .with_option("gt", "Greater Than (>)")
                .with_option("gte", "Greater or Equal (>=)")
                .with_option("lt", "Less Than (<)")
                .with_option("lte", "Less or Equal (<=)")
                .with_option("contains", "Contains")
                .with_option("starts_with", "Starts With")
                .with_option("ends_with", "Ends With")
                .with_option("regex", "Regex Match")
                .with_option("is_null", "Is Null")
                .with_option("is_empty", "Is Empty")
                .with_default("custom")
                .with_description("Quick operator selection (overrides expression)"),
        )
        .with_config(
            ConfigOption::string("compare_value", "Compare Value")
                .with_description("Value to compare against (for quick operators)"),
        )
        .with_config(
            ConfigOption::boolean("case_sensitive", "Case Sensitive")
                .with_description("Case-sensitive string comparisons")
                .with_default(true),
        )
        .with_config(
            ConfigOption::boolean("coerce_types", "Coerce Types")
                .with_description("Attempt type coercion for comparisons")
                .with_default(false),
        )
        .with_default_size(180.0, 140.0)
        .with_generator("logic::condition")
}

/// Create a Switch/Match component definition
///
/// A Switch component evaluates a value and routes to one of multiple outputs
/// based on matching cases.
pub fn switch_component() -> ComponentDefinition {
    ComponentDefinition::new("logic.switch", "Switch", ComponentCategory::Logic)
        .with_description("Multi-way branching based on value matching")
        .with_icon("🎛")
        .with_tag("switch")
        .with_tag("match")
        .with_tag("case")
        .with_tag("flow")
        // Input ports
        .with_input(
            PortDefinition::data_in("value", "Value", DataType::Any)
                .with_description("Value to match against cases")
                .required(),
        )
        .with_input(
            PortDefinition::trigger_in("evaluate", "Evaluate")
                .with_description("Trigger switch evaluation"),
        )
        // Output ports
        .with_output(
            PortDefinition::trigger_out("case_1", "Case 1")
                .with_description("First case output"),
        )
        .with_output(
            PortDefinition::trigger_out("case_2", "Case 2")
                .with_description("Second case output"),
        )
        .with_output(
            PortDefinition::trigger_out("case_3", "Case 3")
                .with_description("Third case output"),
        )
        .with_output(
            PortDefinition::trigger_out("default", "Default")
                .with_description("Default case when no match"),
        )
        .with_output(
            PortDefinition::data_out("value", "Value", DataType::Any)
                .with_description("Pass-through of the input value"),
        )
        .with_output(
            PortDefinition::data_out("matched_case", "Matched Case", DataType::String)
                .with_description("Name of the matched case"),
        )
        // Fields
        .with_field(
            FieldDefinition::text("cases")
                .with_label("Cases (JSON)")
                .with_placeholder(r#"{"case_1": "value1", "case_2": ["value2", "value3"]}"#)
                .with_description("JSON mapping of case names to match values")
                .with_widget("code"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("allow_multiple", "Allow Multiple Matches")
                .with_description("Trigger all matching cases instead of just first")
                .with_default(false),
        )
        .with_config(
            ConfigOption::boolean("case_sensitive", "Case Sensitive")
                .with_description("Case-sensitive string matching")
                .with_default(true),
        )
        .with_config(
            ConfigOption::boolean("require_match", "Require Match")
                .with_description("Error if no case matches (instead of using default)")
                .with_default(false),
        )
        .with_default_size(200.0, 200.0)
        .with_generator("logic::switch")
}

/// Create a Loop/Iterator component definition
///
/// A Loop component iterates over a collection, emitting each item.
pub fn loop_component() -> ComponentDefinition {
    ComponentDefinition::new("logic.loop", "Loop", ComponentCategory::Logic)
        .with_description("Iterate over a collection of items")
        .with_icon("🔁")
        .with_tag("loop")
        .with_tag("iterate")
        .with_tag("foreach")
        .with_tag("flow")
        // Input ports
        .with_input(
            PortDefinition::data_in("items", "Items", DataType::Array(Box::new(DataType::Any)))
                .with_description("Collection to iterate over")
                .required(),
        )
        .with_input(
            PortDefinition::trigger_in("start", "Start")
                .with_description("Start iteration"),
        )
        .with_input(
            PortDefinition::trigger_in("stop", "Stop")
                .with_description("Stop iteration"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("item", "Current Item", DataType::Any)
                .with_description("Current item in iteration"),
        )
        .with_output(
            PortDefinition::data_out("index", "Index", DataType::Int64)
                .with_description("Current index (0-based)"),
        )
        .with_output(
            PortDefinition::data_out("is_first", "Is First", DataType::Bool)
                .with_description("True if this is the first item"),
        )
        .with_output(
            PortDefinition::data_out("is_last", "Is Last", DataType::Bool)
                .with_description("True if this is the last item"),
        )
        .with_output(
            PortDefinition::trigger_out("each", "For Each")
                .with_description("Triggered for each item"),
        )
        .with_output(
            PortDefinition::trigger_out("complete", "On Complete")
                .with_description("Triggered when iteration completes"),
        )
        .with_output(
            PortDefinition::data_out("count", "Total Count", DataType::Int64)
                .with_description("Total number of items"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("parallel", "Parallel Execution")
                .with_description("Process items in parallel (order not guaranteed)")
                .with_default(false),
        )
        .with_config(
            ConfigOption::integer("batch_size", "Batch Size")
                .with_description("Number of items to process in each batch (0 = no batching)")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_min(0.0),
        )
        .with_config(
            ConfigOption::integer("delay_ms", "Delay (ms)")
                .with_description("Delay between iterations in milliseconds")
                .with_default(imortal_core::ConfigValue::Int(0))
                .with_min(0.0),
        )
        .with_config(
            ConfigOption::boolean("continue_on_error", "Continue on Error")
                .with_description("Continue iteration if an error occurs")
                .with_default(false),
        )
        .with_default_size(180.0, 180.0)
        .with_generator("logic::loop")
}

/// Create a Merge component definition
///
/// A Merge component combines multiple inputs into a single output.
pub fn merge_component() -> ComponentDefinition {
    ComponentDefinition::new("logic.merge", "Merge", ComponentCategory::Logic)
        .with_description("Combine multiple data inputs into one")
        .with_icon("🔗")
        .with_tag("merge")
        .with_tag("combine")
        .with_tag("join")
        // Input ports
        .with_input(
            PortDefinition::data_in("input_1", "Input 1", DataType::Any)
                .with_description("First input")
                .multiple(),
        )
        .with_input(
            PortDefinition::data_in("input_2", "Input 2", DataType::Any)
                .with_description("Second input")
                .multiple(),
        )
        .with_input(
            PortDefinition::data_in("input_3", "Input 3", DataType::Any)
                .with_description("Third input")
                .multiple(),
        )
        .with_input(
            PortDefinition::trigger_in("merge", "Merge")
                .with_description("Trigger merge operation"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("output", "Output", DataType::Any)
                .with_description("Merged result"),
        )
        .with_output(
            PortDefinition::trigger_out("complete", "On Complete")
                .with_description("Triggered when merge completes"),
        )
        // Configuration
        .with_config(
            ConfigOption::select("strategy", "Merge Strategy")
                .with_option("object", "Merge Objects")
                .with_option("array", "Concatenate Arrays")
                .with_option("first", "First Non-Null")
                .with_option("last", "Last Non-Null")
                .with_option("custom", "Custom Expression")
                .with_default("object")
                .with_description("How to combine the inputs"),
        )
        .with_config(
            ConfigOption::boolean("deep_merge", "Deep Merge")
                .with_description("Recursively merge nested objects")
                .with_default(true),
        )
        .with_config(
            ConfigOption::select("conflict_resolution", "Conflict Resolution")
                .with_option("last_wins", "Last Wins")
                .with_option("first_wins", "First Wins")
                .with_option("error", "Error on Conflict")
                .with_option("merge_arrays", "Merge Arrays")
                .with_default("last_wins")
                .with_description("How to handle conflicting keys"),
        )
        .with_config(
            ConfigOption::boolean("wait_for_all", "Wait for All")
                .with_description("Wait for all inputs before merging")
                .with_default(true),
        )
        .with_default_size(180.0, 160.0)
        .with_generator("logic::merge")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_component() {
        let def = validator_component();
        assert_eq!(def.id, "logic.validator");
        assert_eq!(def.category, ComponentCategory::Logic);
        assert!(def.ports.inputs.iter().any(|p| p.id == "data"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "valid"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "invalid"));
    }

    #[test]
    fn test_transformer_component() {
        let def = transformer_component();
        assert_eq!(def.id, "logic.transformer");
        assert!(def.ports.inputs.iter().any(|p| p.id == "input"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "output"));
        assert!(def.config.iter().any(|c| c.id == "mode"));
    }

    #[test]
    fn test_condition_component() {
        let def = condition_component();
        assert_eq!(def.id, "logic.condition");
        assert!(def.ports.outputs.iter().any(|p| p.id == "true"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "false"));
        assert!(def.fields.iter().any(|f| f.name == "expression"));
    }

    #[test]
    fn test_switch_component() {
        let def = switch_component();
        assert_eq!(def.id, "logic.switch");
        assert!(def.ports.outputs.iter().any(|p| p.id == "default"));
        assert!(def.config.iter().any(|c| c.id == "allow_multiple"));
    }

    #[test]
    fn test_loop_component() {
        let def = loop_component();
        assert_eq!(def.id, "logic.loop");
        assert!(def.ports.inputs.iter().any(|p| p.id == "items"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "each"));
        assert!(def.ports.outputs.iter().any(|p| p.id == "complete"));
    }

    #[test]
    fn test_merge_component() {
        let def = merge_component();
        assert_eq!(def.id, "logic.merge");
        assert!(def.config.iter().any(|c| c.id == "strategy"));
        assert!(def.config.iter().any(|c| c.id == "deep_merge"));
    }

    #[test]
    fn test_instantiate_condition() {
        let def = condition_component();
        let node = def.instantiate("Age Check");

        assert_eq!(node.name, "Age Check");
        assert_eq!(node.component_type, "logic.condition");
        assert!(!node.ports.outputs.is_empty());
    }
}
