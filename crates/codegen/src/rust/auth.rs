//! Authentication code generation for Immortal Engine
//!
//! This module provides utilities for generating Rust authentication code
//! from auth-related nodes in the project graph (Login, Register, Session, etc.)

use imortal_ir::{Node, ProjectGraph};
use imortal_core::{DataType, EngineResult};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

/// Authentication framework target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AuthFramework {
    #[default]
    Axum,
    Actix,
    Custom,
}

/// Configuration for auth code generation
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Target framework
    pub framework: AuthFramework,
    /// Whether to use JWT tokens
    pub use_jwt: bool,
    /// JWT secret environment variable name
    pub jwt_secret_env: String,
    /// Session duration in seconds
    pub session_duration_secs: u64,
    /// Whether to hash passwords with argon2
    pub use_argon2: bool,
    /// Whether to generate refresh tokens
    pub use_refresh_tokens: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            framework: AuthFramework::Axum,
            use_jwt: true,
            jwt_secret_env: "JWT_SECRET".to_string(),
            session_duration_secs: 86400, // 24 hours
            use_argon2: true,
            use_refresh_tokens: false,
        }
    }
}

impl AuthConfig {
    /// Create config for Axum
    pub fn axum() -> Self {
        Self::default()
    }

    /// Create config for Actix
    pub fn actix() -> Self {
        Self {
            framework: AuthFramework::Actix,
            ..Default::default()
        }
    }

    /// Enable refresh tokens
    pub fn with_refresh_tokens(mut self) -> Self {
        self.use_refresh_tokens = true;
        self
    }

    /// Set session duration
    pub fn with_session_duration(mut self, secs: u64) -> Self {
        self.session_duration_secs = secs;
        self
    }
}

/// Generate authentication code from auth nodes
pub struct AuthGenerator {
    config: AuthConfig,
}

impl AuthGenerator {
    /// Create a new auth generator
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    /// Generate all auth code for a project
    pub fn generate(&self, graph: &ProjectGraph) -> EngineResult<GeneratedAuth> {
        let mut result = GeneratedAuth::new();

        // Find auth nodes
        for node in graph.nodes() {
            match node.component_type.as_str() {
                "auth.login" => {
                    result.login_handler = Some(self.generate_login_handler(node)?);
                }
                "auth.register" => {
                    result.register_handler = Some(self.generate_register_handler(node)?);
                }
                "auth.logout" => {
                    result.logout_handler = Some(self.generate_logout_handler(node)?);
                }
                "auth.session" => {
                    result.session_code = Some(self.generate_session_code(node)?);
                }
                _ => {}
            }
        }

        // Generate common auth utilities
        result.auth_utils = self.generate_auth_utils();
        result.auth_types = self.generate_auth_types();
        result.auth_middleware = self.generate_auth_middleware();

        Ok(result)
    }

    /// Generate login handler
    fn generate_login_handler(&self, node: &Node) -> EngineResult<String> {
        let handler_name = format_ident!("{}", to_snake_case(&node.name));

        // Get configured fields
        let email_field = node.fields.iter()
            .find(|f| f.name == "email" || f.name == "username")
            .map(|f| f.name.as_str())
            .unwrap_or("email");

        let email_ident = format_ident!("{}", email_field);

        let tokens = match self.config.framework {
            AuthFramework::Axum => {
                if self.config.use_jwt {
                    quote! {
                        /// Login handler - authenticates user and returns JWT token
                        pub async fn #handler_name(
                            State(state): State<AppState>,
                            Json(payload): Json<LoginRequest>,
                        ) -> Result<Json<LoginResponse>, AuthError> {
                            // Find user by email
                            let user = state.db.find_user_by_email(&payload.#email_ident)
                                .await
                                .map_err(|_| AuthError::InvalidCredentials)?
                                .ok_or(AuthError::InvalidCredentials)?;

                            // Verify password
                            if !verify_password(&payload.password, &user.password_hash)? {
                                return Err(AuthError::InvalidCredentials);
                            }

                            // Generate JWT token
                            let token = generate_jwt_token(&user, &state.jwt_secret)?;

                            Ok(Json(LoginResponse {
                                token,
                                user: user.into(),
                            }))
                        }

                        #[derive(Debug, Deserialize)]
                        pub struct LoginRequest {
                            pub #email_ident: String,
                            pub password: String,
                        }

                        #[derive(Debug, Serialize)]
                        pub struct LoginResponse {
                            pub token: String,
                            pub user: UserResponse,
                        }
                    }
                } else {
                    quote! {
                        /// Login handler - authenticates user with session
                        pub async fn #handler_name(
                            State(state): State<AppState>,
                            session: Session,
                            Json(payload): Json<LoginRequest>,
                        ) -> Result<Json<LoginResponse>, AuthError> {
                            // Find user by email
                            let user = state.db.find_user_by_email(&payload.#email_ident)
                                .await
                                .map_err(|_| AuthError::InvalidCredentials)?
                                .ok_or(AuthError::InvalidCredentials)?;

                            // Verify password
                            if !verify_password(&payload.password, &user.password_hash)? {
                                return Err(AuthError::InvalidCredentials);
                            }

                            // Set session
                            session.insert("user_id", user.id).await?;

                            Ok(Json(LoginResponse {
                                user: user.into(),
                            }))
                        }

                        #[derive(Debug, Deserialize)]
                        pub struct LoginRequest {
                            pub #email_ident: String,
                            pub password: String,
                        }

                        #[derive(Debug, Serialize)]
                        pub struct LoginResponse {
                            pub user: UserResponse,
                        }
                    }
                }
            }
            AuthFramework::Actix => {
                quote! {
                    /// Login handler - authenticates user and returns JWT token
                    pub async fn #handler_name(
                        state: web::Data<AppState>,
                        payload: web::Json<LoginRequest>,
                    ) -> Result<HttpResponse, AuthError> {
                        // Find user by email
                        let user = state.db.find_user_by_email(&payload.#email_ident)
                            .await
                            .map_err(|_| AuthError::InvalidCredentials)?
                            .ok_or(AuthError::InvalidCredentials)?;

                        // Verify password
                        if !verify_password(&payload.password, &user.password_hash)? {
                            return Err(AuthError::InvalidCredentials);
                        }

                        // Generate JWT token
                        let token = generate_jwt_token(&user, &state.jwt_secret)?;

                        Ok(HttpResponse::Ok().json(LoginResponse {
                            token,
                            user: user.into(),
                        }))
                    }

                    #[derive(Debug, Deserialize)]
                    pub struct LoginRequest {
                        pub #email_ident: String,
                        pub password: String,
                    }

                    #[derive(Debug, Serialize)]
                    pub struct LoginResponse {
                        pub token: String,
                        pub user: UserResponse,
                    }
                }
            }
            AuthFramework::Custom => {
                quote! {
                    /// Login function - authenticates user credentials
                    pub async fn #handler_name(
                        db: &Database,
                        #email_ident: &str,
                        password: &str,
                    ) -> Result<AuthResult, AuthError> {
                        // Find user by email
                        let user = db.find_user_by_email(#email_ident)
                            .await?
                            .ok_or(AuthError::InvalidCredentials)?;

                        // Verify password
                        if !verify_password(password, &user.password_hash)? {
                            return Err(AuthError::InvalidCredentials);
                        }

                        Ok(AuthResult {
                            user,
                            authenticated: true,
                        })
                    }
                }
            }
        };

        Ok(tokens.to_string())
    }

    /// Generate register handler
    fn generate_register_handler(&self, node: &Node) -> EngineResult<String> {
        let handler_name = format_ident!("{}", to_snake_case(&node.name));

        // Build fields from node
        let field_names: Vec<_> = node.fields.iter()
            .filter(|f| f.name != "id" && f.name != "created_at" && f.name != "updated_at")
            .map(|f| format_ident!("{}", to_snake_case(&f.name)))
            .collect();

        let field_types: Vec<_> = node.fields.iter()
            .filter(|f| f.name != "id" && f.name != "created_at" && f.name != "updated_at")
            .map(|f| data_type_to_token(&f.data_type))
            .collect();

        let tokens = match self.config.framework {
            AuthFramework::Axum => {
                quote! {
                    /// Register handler - creates a new user account
                    pub async fn #handler_name(
                        State(state): State<AppState>,
                        Json(payload): Json<RegisterRequest>,
                    ) -> Result<Json<RegisterResponse>, AuthError> {
                        // Check if user already exists
                        if state.db.find_user_by_email(&payload.email).await?.is_some() {
                            return Err(AuthError::UserAlreadyExists);
                        }

                        // Hash password
                        let password_hash = hash_password(&payload.password)?;

                        // Create user
                        let user = state.db.create_user(CreateUser {
                            email: payload.email,
                            password_hash,
                            #(#field_names: payload.#field_names,)*
                        }).await?;

                        // Generate JWT token
                        let token = generate_jwt_token(&user, &state.jwt_secret)?;

                        Ok(Json(RegisterResponse {
                            token,
                            user: user.into(),
                        }))
                    }

                    #[derive(Debug, Deserialize)]
                    pub struct RegisterRequest {
                        pub email: String,
                        pub password: String,
                        #(pub #field_names: #field_types,)*
                    }

                    #[derive(Debug, Serialize)]
                    pub struct RegisterResponse {
                        pub token: String,
                        pub user: UserResponse,
                    }
                }
            }
            AuthFramework::Actix => {
                quote! {
                    /// Register handler - creates a new user account
                    pub async fn #handler_name(
                        state: web::Data<AppState>,
                        payload: web::Json<RegisterRequest>,
                    ) -> Result<HttpResponse, AuthError> {
                        // Check if user already exists
                        if state.db.find_user_by_email(&payload.email).await?.is_some() {
                            return Err(AuthError::UserAlreadyExists);
                        }

                        // Hash password
                        let password_hash = hash_password(&payload.password)?;

                        // Create user
                        let user = state.db.create_user(CreateUser {
                            email: payload.email.clone(),
                            password_hash,
                            #(#field_names: payload.#field_names.clone(),)*
                        }).await?;

                        // Generate JWT token
                        let token = generate_jwt_token(&user, &state.jwt_secret)?;

                        Ok(HttpResponse::Created().json(RegisterResponse {
                            token,
                            user: user.into(),
                        }))
                    }

                    #[derive(Debug, Deserialize)]
                    pub struct RegisterRequest {
                        pub email: String,
                        pub password: String,
                        #(pub #field_names: #field_types,)*
                    }

                    #[derive(Debug, Serialize)]
                    pub struct RegisterResponse {
                        pub token: String,
                        pub user: UserResponse,
                    }
                }
            }
            AuthFramework::Custom => {
                quote! {
                    /// Register function - creates a new user account
                    pub async fn #handler_name(
                        db: &Database,
                        email: &str,
                        password: &str,
                        #(#field_names: #field_types,)*
                    ) -> Result<User, AuthError> {
                        // Check if user already exists
                        if db.find_user_by_email(email).await?.is_some() {
                            return Err(AuthError::UserAlreadyExists);
                        }

                        // Hash password
                        let password_hash = hash_password(password)?;

                        // Create user
                        let user = db.create_user(CreateUser {
                            email: email.to_string(),
                            password_hash,
                            #(#field_names,)*
                        }).await?;

                        Ok(user)
                    }
                }
            }
        };

        Ok(tokens.to_string())
    }

    /// Generate logout handler
    fn generate_logout_handler(&self, node: &Node) -> EngineResult<String> {
        let handler_name = format_ident!("{}", to_snake_case(&node.name));

        let tokens = match self.config.framework {
            AuthFramework::Axum => {
                if self.config.use_jwt {
                    quote! {
                        /// Logout handler - invalidates the JWT token (client-side)
                        pub async fn #handler_name(
                            State(state): State<AppState>,
                            auth: AuthUser,
                        ) -> Result<Json<LogoutResponse>, AuthError> {
                            // With JWT, logout is typically handled client-side
                            // Optionally add token to a blocklist
                            if let Some(ref blocklist) = state.token_blocklist {
                                blocklist.add(&auth.token).await;
                            }

                            Ok(Json(LogoutResponse {
                                message: "Logged out successfully".to_string(),
                            }))
                        }

                        #[derive(Debug, Serialize)]
                        pub struct LogoutResponse {
                            pub message: String,
                        }
                    }
                } else {
                    quote! {
                        /// Logout handler - destroys the session
                        pub async fn #handler_name(
                            session: Session,
                        ) -> Result<Json<LogoutResponse>, AuthError> {
                            session.purge();

                            Ok(Json(LogoutResponse {
                                message: "Logged out successfully".to_string(),
                            }))
                        }

                        #[derive(Debug, Serialize)]
                        pub struct LogoutResponse {
                            pub message: String,
                        }
                    }
                }
            }
            AuthFramework::Actix => {
                quote! {
                    /// Logout handler
                    pub async fn #handler_name(
                        session: Session,
                    ) -> Result<HttpResponse, AuthError> {
                        session.purge();

                        Ok(HttpResponse::Ok().json(serde_json::json!({
                            "message": "Logged out successfully"
                        })))
                    }
                }
            }
            AuthFramework::Custom => {
                quote! {
                    /// Logout function
                    pub async fn #handler_name(session: &mut Session) -> Result<(), AuthError> {
                        session.invalidate();
                        Ok(())
                    }
                }
            }
        };

        Ok(tokens.to_string())
    }

    /// Generate session management code
    fn generate_session_code(&self, _node: &Node) -> EngineResult<String> {
        let duration_secs = self.config.session_duration_secs;

        let tokens = quote! {
            /// Session configuration
            pub struct SessionConfig {
                /// Session duration in seconds
                pub duration_secs: u64,
                /// Cookie name
                pub cookie_name: &'static str,
                /// Whether the cookie is secure (HTTPS only)
                pub secure: bool,
                /// Whether the cookie is HTTP only
                pub http_only: bool,
                /// Same site policy
                pub same_site: SameSite,
            }

            impl Default for SessionConfig {
                fn default() -> Self {
                    Self {
                        duration_secs: #duration_secs,
                        cookie_name: "session_id",
                        secure: true,
                        http_only: true,
                        same_site: SameSite::Strict,
                    }
                }
            }

            /// Session data stored in the session store
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct SessionData {
                pub user_id: uuid::Uuid,
                pub created_at: chrono::DateTime<chrono::Utc>,
                pub expires_at: chrono::DateTime<chrono::Utc>,
                pub ip_address: Option<String>,
                pub user_agent: Option<String>,
            }

            impl SessionData {
                pub fn new(user_id: uuid::Uuid) -> Self {
                    let now = chrono::Utc::now();
                    Self {
                        user_id,
                        created_at: now,
                        expires_at: now + chrono::Duration::seconds(#duration_secs as i64),
                        ip_address: None,
                        user_agent: None,
                    }
                }

                pub fn is_expired(&self) -> bool {
                    chrono::Utc::now() > self.expires_at
                }

                pub fn refresh(&mut self) {
                    self.expires_at = chrono::Utc::now() + chrono::Duration::seconds(#duration_secs as i64);
                }
            }
        };

        Ok(tokens.to_string())
    }

    /// Generate common auth utilities
    fn generate_auth_utils(&self) -> String {
        let jwt_secret_env = &self.config.jwt_secret_env;
        let duration_secs = self.config.session_duration_secs;

        let password_utils = if self.config.use_argon2 {
            quote! {
                use argon2::{
                    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
                    Argon2,
                };

                /// Hash a password using Argon2
                pub fn hash_password(password: &str) -> Result<String, AuthError> {
                    let salt = SaltString::generate(&mut OsRng);
                    let argon2 = Argon2::default();

                    argon2
                        .hash_password(password.as_bytes(), &salt)
                        .map(|hash| hash.to_string())
                        .map_err(|_| AuthError::PasswordHashError)
                }

                /// Verify a password against a hash
                pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
                    let parsed_hash = PasswordHash::new(hash)
                        .map_err(|_| AuthError::PasswordHashError)?;

                    Ok(Argon2::default()
                        .verify_password(password.as_bytes(), &parsed_hash)
                        .is_ok())
                }
            }
        } else {
            quote! {
                use bcrypt::{hash, verify, DEFAULT_COST};

                /// Hash a password using bcrypt
                pub fn hash_password(password: &str) -> Result<String, AuthError> {
                    hash(password, DEFAULT_COST)
                        .map_err(|_| AuthError::PasswordHashError)
                }

                /// Verify a password against a hash
                pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
                    verify(password, hash)
                        .map_err(|_| AuthError::PasswordHashError)
                }
            }
        };

        let jwt_utils = if self.config.use_jwt {
            quote! {
                use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

                /// JWT Claims
                #[derive(Debug, Serialize, Deserialize)]
                pub struct Claims {
                    pub sub: String,  // user_id
                    pub email: String,
                    pub exp: usize,   // expiration timestamp
                    pub iat: usize,   // issued at
                }

                /// Generate a JWT token for a user
                pub fn generate_jwt_token(user: &User, secret: &str) -> Result<String, AuthError> {
                    let now = chrono::Utc::now();
                    let exp = (now + chrono::Duration::seconds(#duration_secs as i64)).timestamp() as usize;
                    let iat = now.timestamp() as usize;

                    let claims = Claims {
                        sub: user.id.to_string(),
                        email: user.email.clone(),
                        exp,
                        iat,
                    };

                    encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret.as_bytes()),
                    )
                    .map_err(|_| AuthError::TokenGenerationError)
                }

                /// Decode and validate a JWT token
                pub fn decode_jwt_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
                    decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &Validation::new(Algorithm::HS256),
                    )
                    .map(|data| data.claims)
                    .map_err(|_| AuthError::InvalidToken)
                }

                /// Get JWT secret from environment
                pub fn get_jwt_secret() -> String {
                    std::env::var(#jwt_secret_env)
                        .expect(&format!("{} must be set", #jwt_secret_env))
                }
            }
        } else {
            quote! {}
        };

        let tokens = quote! {
            //! Authentication utilities

            #password_utils

            #jwt_utils

            /// Validate email format
            pub fn is_valid_email(email: &str) -> bool {
                // Simple email validation
                email.contains('@') && email.contains('.') && email.len() > 5
            }

            /// Validate password strength
            pub fn validate_password_strength(password: &str) -> Result<(), &'static str> {
                if password.len() < 8 {
                    return Err("Password must be at least 8 characters long");
                }
                if !password.chars().any(|c| c.is_uppercase()) {
                    return Err("Password must contain at least one uppercase letter");
                }
                if !password.chars().any(|c| c.is_lowercase()) {
                    return Err("Password must contain at least one lowercase letter");
                }
                if !password.chars().any(|c| c.is_numeric()) {
                    return Err("Password must contain at least one number");
                }
                Ok(())
            }
        };

        tokens.to_string()
    }

    /// Generate auth types (errors, responses)
    fn generate_auth_types(&self) -> String {
        let tokens = quote! {
            //! Authentication types and error definitions

            use thiserror::Error;

            /// Authentication errors
            #[derive(Debug, Error)]
            pub enum AuthError {
                #[error("Invalid credentials")]
                InvalidCredentials,

                #[error("User already exists")]
                UserAlreadyExists,

                #[error("User not found")]
                UserNotFound,

                #[error("Invalid token")]
                InvalidToken,

                #[error("Token expired")]
                TokenExpired,

                #[error("Token generation failed")]
                TokenGenerationError,

                #[error("Password hash error")]
                PasswordHashError,

                #[error("Unauthorized")]
                Unauthorized,

                #[error("Forbidden")]
                Forbidden,

                #[error("Session error: {0}")]
                SessionError(String),

                #[error("Database error: {0}")]
                DatabaseError(String),

                #[error("Internal error: {0}")]
                Internal(String),
            }

            /// Authenticated user information extracted from token/session
            #[derive(Debug, Clone)]
            pub struct AuthUser {
                pub id: uuid::Uuid,
                pub email: String,
                pub token: String,
            }

            /// User response (safe to send to client)
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct UserResponse {
                pub id: uuid::Uuid,
                pub email: String,
                pub created_at: chrono::DateTime<chrono::Utc>,
            }

            impl From<User> for UserResponse {
                fn from(user: User) -> Self {
                    Self {
                        id: user.id,
                        email: user.email,
                        created_at: user.created_at,
                    }
                }
            }
        };

        tokens.to_string()
    }

    /// Generate auth middleware
    fn generate_auth_middleware(&self) -> String {
        let tokens = match self.config.framework {
            AuthFramework::Axum => {
                quote! {
                    //! Authentication middleware for Axum

                    use axum::{
                        async_trait,
                        extract::FromRequestParts,
                        http::{request::Parts, StatusCode},
                    };

                    #[async_trait]
                    impl<S> FromRequestParts<S> for AuthUser
                    where
                        S: Send + Sync,
                    {
                        type Rejection = (StatusCode, &'static str);

                        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
                            // Extract Authorization header
                            let auth_header = parts
                                .headers
                                .get("Authorization")
                                .and_then(|value| value.to_str().ok())
                                .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

                            // Extract Bearer token
                            let token = auth_header
                                .strip_prefix("Bearer ")
                                .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization header format"))?;

                            // Decode token
                            let secret = get_jwt_secret();
                            let claims = decode_jwt_token(token, &secret)
                                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

                            Ok(AuthUser {
                                id: uuid::Uuid::parse_str(&claims.sub)
                                    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token"))?,
                                email: claims.email,
                                token: token.to_string(),
                            })
                        }
                    }

                    /// Optional auth extractor - doesn't fail if no auth is present
                    pub struct OptionalAuthUser(pub Option<AuthUser>);

                    #[async_trait]
                    impl<S> FromRequestParts<S> for OptionalAuthUser
                    where
                        S: Send + Sync,
                    {
                        type Rejection = std::convert::Infallible;

                        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
                            Ok(OptionalAuthUser(
                                AuthUser::from_request_parts(parts, state).await.ok()
                            ))
                        }
                    }
                }
            }
            AuthFramework::Actix => {
                quote! {
                    //! Authentication middleware for Actix-web

                    use actix_web::{dev::ServiceRequest, Error, HttpMessage};
                    use actix_web_httpauth::extractors::bearer::BearerAuth;

                    /// Validate JWT token in actix middleware
                    pub async fn jwt_validator(
                        req: ServiceRequest,
                        credentials: BearerAuth,
                    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
                        let secret = get_jwt_secret();
                        let token = credentials.token();

                        match decode_jwt_token(token, &secret) {
                            Ok(claims) => {
                                let auth_user = AuthUser {
                                    id: uuid::Uuid::parse_str(&claims.sub).unwrap(),
                                    email: claims.email,
                                    token: token.to_string(),
                                };
                                req.extensions_mut().insert(auth_user);
                                Ok(req)
                            }
                            Err(_) => {
                                Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
                            }
                        }
                    }
                }
            }
            AuthFramework::Custom => {
                quote! {
                    //! Custom authentication middleware

                    /// Middleware trait for custom implementations
                    #[async_trait::async_trait]
                    pub trait AuthMiddleware {
                        async fn authenticate(&self, token: &str) -> Result<AuthUser, AuthError>;
                    }

                    /// Default JWT-based authentication middleware
                    pub struct JwtAuthMiddleware {
                        secret: String,
                    }

                    impl JwtAuthMiddleware {
                        pub fn new(secret: impl Into<String>) -> Self {
                            Self { secret: secret.into() }
                        }
                    }

                    #[async_trait::async_trait]
                    impl AuthMiddleware for JwtAuthMiddleware {
                        async fn authenticate(&self, token: &str) -> Result<AuthUser, AuthError> {
                            let claims = decode_jwt_token(token, &self.secret)?;

                            Ok(AuthUser {
                                id: uuid::Uuid::parse_str(&claims.sub)
                                    .map_err(|_| AuthError::InvalidToken)?,
                                email: claims.email,
                                token: token.to_string(),
                            })
                        }
                    }
                }
            }
        };

        tokens.to_string()
    }
}

/// Result of auth code generation
#[derive(Debug, Clone, Default)]
pub struct GeneratedAuth {
    /// Login handler code
    pub login_handler: Option<String>,
    /// Register handler code
    pub register_handler: Option<String>,
    /// Logout handler code
    pub logout_handler: Option<String>,
    /// Session management code
    pub session_code: Option<String>,
    /// Auth utilities
    pub auth_utils: String,
    /// Auth types (errors, responses)
    pub auth_types: String,
    /// Auth middleware code
    pub auth_middleware: String,
}

impl GeneratedAuth {
    /// Create a new empty result
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any auth code was generated
    pub fn has_auth_code(&self) -> bool {
        self.login_handler.is_some()
            || self.register_handler.is_some()
            || self.logout_handler.is_some()
    }

    /// Get all generated code as a single module
    pub fn to_module(&self) -> String {
        let mut output = String::new();

        output.push_str("//! Authentication module - Generated by Immortal Engine\n\n");
        output.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Add types
        output.push_str("// ========== Auth Types ==========\n\n");
        output.push_str(&self.auth_types);
        output.push_str("\n\n");

        // Add utilities
        output.push_str("// ========== Auth Utilities ==========\n\n");
        output.push_str(&self.auth_utils);
        output.push_str("\n\n");

        // Add middleware
        output.push_str("// ========== Auth Middleware ==========\n\n");
        output.push_str(&self.auth_middleware);
        output.push_str("\n\n");

        // Add handlers
        output.push_str("// ========== Auth Handlers ==========\n\n");

        if let Some(ref login) = self.login_handler {
            output.push_str(login);
            output.push_str("\n\n");
        }

        if let Some(ref register) = self.register_handler {
            output.push_str(register);
            output.push_str("\n\n");
        }

        if let Some(ref logout) = self.logout_handler {
            output.push_str(logout);
            output.push_str("\n\n");
        }

        if let Some(ref session) = self.session_code {
            output.push_str("// ========== Session Management ==========\n\n");
            output.push_str(session);
            output.push_str("\n");
        }

        output
    }
}

/// Convert DataType to TokenStream for code generation
fn data_type_to_token(data_type: &DataType) -> TokenStream {
    match data_type {
        DataType::String | DataType::Text => quote! { String },
        DataType::Int32 => quote! { i32 },
        DataType::Int64 => quote! { i64 },
        DataType::Float32 => quote! { f32 },
        DataType::Float64 => quote! { f64 },
        DataType::Bool => quote! { bool },
        DataType::Uuid => quote! { uuid::Uuid },
        DataType::DateTime => quote! { chrono::DateTime<chrono::Utc> },
        DataType::Optional(inner) => {
            let inner_type = data_type_to_token(inner);
            quote! { Option<#inner_type> }
        }
        DataType::Array(inner) => {
            let inner_type = data_type_to_token(inner);
            quote! { Vec<#inner_type> }
        }
        _ => quote! { String },
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

/// Generate auth routes for router configuration
pub fn generate_auth_routes(framework: AuthFramework) -> String {
    match framework {
        AuthFramework::Axum => {
            let tokens = quote! {
                /// Create auth router with all authentication routes
                pub fn auth_routes() -> axum::Router<AppState> {
                    axum::Router::new()
                        .route("/login", axum::routing::post(login))
                        .route("/register", axum::routing::post(register))
                        .route("/logout", axum::routing::post(logout))
                        .route("/me", axum::routing::get(get_current_user))
                }
            };
            tokens.to_string()
        }
        AuthFramework::Actix => {
            let tokens = quote! {
                /// Configure auth routes for Actix
                pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
                    cfg.service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                            .route("/register", web::post().to(register))
                            .route("/logout", web::post().to(logout))
                            .route("/me", web::get().to(get_current_user))
                    );
                }
            };
            tokens.to_string()
        }
        AuthFramework::Custom => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use imortal_ir::{ProjectMeta, ProjectGraph, Node, Field};

    fn create_test_graph() -> ProjectGraph {
        let meta = ProjectMeta::new("test_app");
        let mut graph = ProjectGraph::new(meta);

        // Add login node
        let login = Node::new_login()
            .with_field(Field::string("email").required())
            .with_field(Field::string("password").required().secret());
        graph.add_node(login);

        // Add register node
        let register = Node::new_register()
            .with_field(Field::string("email").required())
            .with_field(Field::string("password").required().secret())
            .with_field(Field::string("name").required());
        graph.add_node(register);

        graph
    }

    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();
        assert!(config.use_jwt);
        assert!(config.use_argon2);
        assert_eq!(config.framework, AuthFramework::Axum);
    }

    #[test]
    fn test_auth_config_actix() {
        let config = AuthConfig::actix();
        assert_eq!(config.framework, AuthFramework::Actix);
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("LoginHandler"), "login_handler");
        assert_eq!(to_snake_case("login"), "login");
        assert_eq!(to_snake_case("UserAuth"), "user_auth");
    }

    #[test]
    fn test_generated_auth_to_module() {
        let auth = GeneratedAuth {
            login_handler: Some("pub async fn login() {}".to_string()),
            register_handler: None,
            logout_handler: None,
            session_code: None,
            auth_utils: "// utils".to_string(),
            auth_types: "// types".to_string(),
            auth_middleware: "// middleware".to_string(),
        };

        let module = auth.to_module();
        assert!(module.contains("Authentication module"));
        assert!(module.contains("pub async fn login"));
    }

    #[test]
    fn test_generate_auth_routes_axum() {
        let routes = generate_auth_routes(AuthFramework::Axum);
        assert!(routes.contains("auth_routes"));
        assert!(routes.contains("/login"));
    }
}
