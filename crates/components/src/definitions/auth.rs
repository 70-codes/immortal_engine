//! Authentication Component Definitions
//!
//! This module provides component definitions for authentication-related functionality:
//! - Login: User login with email/password
//! - Register: User registration
//! - Logout: User logout/session termination
//! - Session: Session management

use crate::definition::{
    ComponentDefinition, ConfigOption, ConfigType, FieldDefinition, PortDefinition,
};
use imortal_core::{ComponentCategory, DataType, Validation};

/// Create the Login component definition
pub fn login_component() -> ComponentDefinition {
    ComponentDefinition::new("auth.login", "Login", ComponentCategory::Auth)
        .with_description("User login component with email and password authentication")
        .with_icon("🔐")
        .with_tag("authentication")
        .with_tag("user")
        // Fields
        .with_field(
            FieldDefinition::string("email")
                .required()
                .with_label("Email")
                .with_placeholder("Enter your email")
                .with_validation(Validation::Email)
                .with_order(1),
        )
        .with_field(
            FieldDefinition::string("password")
                .required()
                .secret()
                .with_label("Password")
                .with_placeholder("Enter your password")
                .with_validation(Validation::MinLength(8))
                .with_order(2),
        )
        .with_field(
            FieldDefinition::bool("remember_me")
                .with_label("Remember Me")
                .with_default(false)
                .with_order(3),
        )
        // Input ports
        .with_input(PortDefinition::trigger_in("submit", "Submit").with_description("Trigger login attempt"))
        .with_input(
            PortDefinition::data_in("credentials", "Credentials", DataType::Any)
                .with_description("Optional pre-filled credentials"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("user", "User", DataType::Entity("User".to_string()))
                .with_description("The authenticated user on success"),
        )
        .with_output(
            PortDefinition::data_out("token", "Token", DataType::String)
                .with_description("Authentication token/session ID"),
        )
        .with_output(
            PortDefinition::trigger_out("success", "On Success")
                .with_description("Triggered when login succeeds"),
        )
        .with_output(
            PortDefinition::trigger_out("failure", "On Failure")
                .with_description("Triggered when login fails"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message on failure"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("require_email_verification", "Require Email Verification")
                .with_default(false)
                .with_description("Require email to be verified before allowing login"),
        )
        .with_config(
            ConfigOption::integer("max_attempts", "Max Attempts")
                .with_default(imortal_core::ConfigValue::Int(5))
                .with_min(1.0)
                .with_max(10.0)
                .with_description("Maximum login attempts before lockout"),
        )
        .with_config(
            ConfigOption::duration("lockout_duration", "Lockout Duration")
                .with_default("15m")
                .with_description("Duration of account lockout after max attempts"),
        )
        .with_config(
            ConfigOption::duration("session_duration", "Session Duration")
                .with_default("24h")
                .with_description("How long the session remains valid"),
        )
        .with_generator("auth::login")
}

/// Create the Register component definition
pub fn register_component() -> ComponentDefinition {
    ComponentDefinition::new("auth.register", "Register", ComponentCategory::Auth)
        .with_description("User registration component for creating new accounts")
        .with_icon("📝")
        .with_tag("authentication")
        .with_tag("user")
        .with_tag("signup")
        // Fields
        .with_field(
            FieldDefinition::string("username")
                .required()
                .with_label("Username")
                .with_placeholder("Choose a username")
                .with_validation(Validation::MinLength(3))
                .with_validation(Validation::MaxLength(32))
                .with_order(1),
        )
        .with_field(
            FieldDefinition::string("email")
                .required()
                .with_label("Email")
                .with_placeholder("Enter your email")
                .with_validation(Validation::Email)
                .with_order(2),
        )
        .with_field(
            FieldDefinition::string("password")
                .required()
                .secret()
                .with_label("Password")
                .with_placeholder("Create a password")
                .with_validation(Validation::MinLength(8))
                .with_order(3),
        )
        .with_field(
            FieldDefinition::string("confirm_password")
                .required()
                .secret()
                .with_label("Confirm Password")
                .with_placeholder("Confirm your password")
                .with_order(4),
        )
        .with_field(
            FieldDefinition::bool("accept_terms")
                .required()
                .with_label("Accept Terms & Conditions")
                .with_default(false)
                .with_order(5),
        )
        // Input ports
        .with_input(PortDefinition::trigger_in("submit", "Submit").with_description("Trigger registration"))
        .with_input(
            PortDefinition::data_in("user_data", "User Data", DataType::Any)
                .with_description("Additional user data to include"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("user", "User", DataType::Entity("User".to_string()))
                .with_description("The newly created user"),
        )
        .with_output(
            PortDefinition::trigger_out("success", "On Success")
                .with_description("Triggered when registration succeeds"),
        )
        .with_output(
            PortDefinition::trigger_out("failure", "On Failure")
                .with_description("Triggered when registration fails"),
        )
        .with_output(
            PortDefinition::data_out("error", "Error", DataType::String)
                .with_description("Error message on failure"),
        )
        .with_output(
            PortDefinition::data_out("validation_errors", "Validation Errors", DataType::Json)
                .with_description("Field-level validation errors"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("require_email_verification", "Require Email Verification")
                .with_default(true)
                .with_description("Send verification email before activating account"),
        )
        .with_config(
            ConfigOption::boolean("auto_login", "Auto Login After Register")
                .with_default(true)
                .with_description("Automatically log in user after successful registration"),
        )
        .with_config(
            ConfigOption::integer("min_password_length", "Minimum Password Length")
                .with_default(imortal_core::ConfigValue::Int(8))
                .with_min(6.0)
                .with_max(128.0),
        )
        .with_config(
            ConfigOption::boolean("require_password_uppercase", "Require Uppercase")
                .with_default(false)
                .with_description("Password must contain uppercase letter"),
        )
        .with_config(
            ConfigOption::boolean("require_password_number", "Require Number")
                .with_default(false)
                .with_description("Password must contain a number"),
        )
        .with_config(
            ConfigOption::boolean("require_password_special", "Require Special Character")
                .with_default(false)
                .with_description("Password must contain a special character"),
        )
        .with_generator("auth::register")
}

/// Create the Logout component definition
pub fn logout_component() -> ComponentDefinition {
    ComponentDefinition::new("auth.logout", "Logout", ComponentCategory::Auth)
        .with_description("User logout component for ending sessions")
        .with_icon("🚪")
        .with_tag("authentication")
        .with_tag("session")
        // Input ports
        .with_input(PortDefinition::trigger_in("logout", "Logout").with_description("Trigger logout"))
        .with_input(
            PortDefinition::data_in("session", "Session", DataType::Any)
                .with_description("Session to terminate (uses current if not provided)"),
        )
        // Output ports
        .with_output(
            PortDefinition::trigger_out("success", "On Success")
                .with_description("Triggered when logout succeeds"),
        )
        .with_output(
            PortDefinition::trigger_out("complete", "On Complete")
                .with_description("Triggered when logout completes (success or failure)"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("invalidate_all_sessions", "Invalidate All Sessions")
                .with_default(false)
                .with_description("Log out from all devices, not just current"),
        )
        .with_config(
            ConfigOption::string("redirect_url", "Redirect URL")
                .with_default("/")
                .with_description("URL to redirect to after logout"),
        )
        .with_config(
            ConfigOption::boolean("clear_cookies", "Clear Cookies")
                .with_default(true)
                .with_description("Clear authentication cookies on logout"),
        )
        .with_generator("auth::logout")
        .with_default_size(150.0, 100.0)
}

/// Create the Session component definition
pub fn session_component() -> ComponentDefinition {
    ComponentDefinition::new("auth.session", "Session", ComponentCategory::Auth)
        .with_description("Session management component for checking and managing user sessions")
        .with_icon("🎫")
        .with_tag("authentication")
        .with_tag("session")
        .with_tag("middleware")
        // Input ports
        .with_input(
            PortDefinition::trigger_in("check", "Check Session")
                .with_description("Check if current session is valid"),
        )
        .with_input(
            PortDefinition::trigger_in("refresh", "Refresh Session")
                .with_description("Refresh the session token"),
        )
        .with_input(
            PortDefinition::data_in("token", "Token", DataType::String)
                .with_description("Session token to validate"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("user", "User", DataType::Entity("User".to_string()))
                .with_description("The authenticated user if session is valid"),
        )
        .with_output(
            PortDefinition::data_out("session_data", "Session Data", DataType::Json)
                .with_description("Session metadata and state"),
        )
        .with_output(
            PortDefinition::trigger_out("valid", "Session Valid")
                .with_description("Triggered when session is valid"),
        )
        .with_output(
            PortDefinition::trigger_out("invalid", "Session Invalid")
                .with_description("Triggered when session is invalid or expired"),
        )
        .with_output(
            PortDefinition::trigger_out("refreshed", "Session Refreshed")
                .with_description("Triggered when session is successfully refreshed"),
        )
        .with_output(
            PortDefinition::data_out("new_token", "New Token", DataType::String)
                .with_description("New token if session was refreshed"),
        )
        // Configuration
        .with_config(
            ConfigOption::boolean("auto_refresh", "Auto Refresh")
                .with_default(true)
                .with_description("Automatically refresh session before expiry"),
        )
        .with_config(
            ConfigOption::duration("refresh_threshold", "Refresh Threshold")
                .with_default("5m")
                .with_description("Refresh session when this much time remains"),
        )
        .with_config(
            ConfigOption::select("storage", "Session Storage")
                .with_option("cookie", "Cookie")
                .with_option("local_storage", "Local Storage")
                .with_option("memory", "Memory")
                .with_default("cookie")
                .with_description("Where to store session token"),
        )
        .with_config(
            ConfigOption::boolean("secure_only", "Secure Only")
                .with_default(true)
                .with_description("Only transmit session over HTTPS")
                .advanced(),
        )
        .with_generator("auth::session")
}

/// Create an OAuth component definition
pub fn oauth_component() -> ComponentDefinition {
    ComponentDefinition::new("auth.oauth", "OAuth", ComponentCategory::Auth)
        .with_description("OAuth 2.0 authentication with external providers")
        .with_icon("🔗")
        .with_tag("authentication")
        .with_tag("oauth")
        .with_tag("social")
        // Fields
        .with_field(
            FieldDefinition::string("provider")
                .required()
                .with_label("Provider")
                .with_default("google")
                .with_order(1),
        )
        // Input ports
        .with_input(
            PortDefinition::trigger_in("start", "Start OAuth")
                .with_description("Initiate OAuth flow"),
        )
        .with_input(
            PortDefinition::data_in("callback_data", "Callback Data", DataType::Any)
                .with_description("OAuth callback data"),
        )
        // Output ports
        .with_output(
            PortDefinition::data_out("user", "User", DataType::Entity("User".to_string()))
                .with_description("The authenticated user"),
        )
        .with_output(
            PortDefinition::data_out("access_token", "Access Token", DataType::String)
                .with_description("OAuth access token"),
        )
        .with_output(
            PortDefinition::data_out("provider_data", "Provider Data", DataType::Json)
                .with_description("Raw data from OAuth provider"),
        )
        .with_output(PortDefinition::trigger_out("success", "On Success"))
        .with_output(PortDefinition::trigger_out("failure", "On Failure"))
        .with_output(PortDefinition::data_out("error", "Error", DataType::String))
        // Configuration
        .with_config(
            ConfigOption::select("provider", "OAuth Provider")
                .with_option("google", "Google")
                .with_option("github", "GitHub")
                .with_option("facebook", "Facebook")
                .with_option("twitter", "Twitter/X")
                .with_option("discord", "Discord")
                .with_option("custom", "Custom")
                .with_default("google"),
        )
        .with_config(
            ConfigOption::string("client_id", "Client ID")
                .required()
                .with_description("OAuth client ID (use environment variable)"),
        )
        .with_config(
            ConfigOption::string("client_secret", "Client Secret")
                .required()
                .with_description("OAuth client secret (use environment variable)"),
        )
        .with_config(
            ConfigOption::string("redirect_uri", "Redirect URI")
                .with_default("/auth/callback")
                .with_description("OAuth callback URL"),
        )
        .with_config(
            ConfigOption::string("scopes", "Scopes")
                .with_default("email profile")
                .with_description("OAuth scopes to request (space-separated)"),
        )
        .with_generator("auth::oauth")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_component() {
        let component = login_component();

        assert_eq!(component.id, "auth.login");
        assert_eq!(component.category, ComponentCategory::Auth);
        assert!(component.fields.iter().any(|f| f.name == "email"));
        assert!(component.fields.iter().any(|f| f.name == "password"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "success"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "failure"));
    }

    #[test]
    fn test_register_component() {
        let component = register_component();

        assert_eq!(component.id, "auth.register");
        assert!(component.fields.iter().any(|f| f.name == "username"));
        assert!(component.fields.iter().any(|f| f.name == "confirm_password"));
    }

    #[test]
    fn test_logout_component() {
        let component = logout_component();

        assert_eq!(component.id, "auth.logout");
        assert!(component.ports.inputs.iter().any(|p| p.id == "logout"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "success"));
    }

    #[test]
    fn test_session_component() {
        let component = session_component();

        assert_eq!(component.id, "auth.session");
        assert!(component.ports.inputs.iter().any(|p| p.id == "check"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "valid"));
        assert!(component.ports.outputs.iter().any(|p| p.id == "invalid"));
    }

    #[test]
    fn test_instantiate_login() {
        let component = login_component();
        let node = component.instantiate("User Login");

        assert_eq!(node.name, "User Login");
        assert_eq!(node.component_type, "auth.login");
        assert!(!node.fields.is_empty());
    }
}
