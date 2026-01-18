# Components Reference

This document provides a comprehensive reference for all available components in Immortal Engine.

## Overview

Components are the building blocks of your application. Each component represents a specific functionality that can be connected to other components to create a complete system.

## Component Categories

| Category | Icon | Description |
|----------|------|-------------|
| Authentication | 🔐 | User authentication and session management |
| Data | 📊 | Data modeling and queries |
| API | 🔌 | API endpoints and protocols |
| Storage | 💾 | Data persistence and caching |
| Logic | ⚙ | Business logic and data processing |

---

## 🔐 Authentication Components

### Login

User login component with email and password authentication.

**Component ID:** `auth.login`

**Fields:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| email | String | Yes | User's email address |
| password | String | Yes | User's password (secret) |

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| submit | Input | Trigger | Triggers login attempt |
| user | Output | Entity(User) | Authenticated user data |
| success | Output | Trigger | Fires on successful login |
| failure | Output | Trigger | Fires on failed login |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| session_duration | Duration | 24h | How long sessions last |
| max_attempts | Integer | 5 | Max failed attempts before lockout |

---

### Register

User registration component for creating new accounts.

**Component ID:** `auth.register`

**Fields:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| username | String | Yes | Desired username |
| email | String | Yes | User's email address |
| password | String | Yes | Desired password (secret) |
| confirm_password | String | Yes | Password confirmation |

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| submit | Input | Trigger | Triggers registration |
| user | Output | Entity(User) | Created user data |
| success | Output | Trigger | Fires on successful registration |
| failure | Output | Trigger | Fires on failed registration |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| require_email_verification | Boolean | true | Require email verification |
| min_password_length | Integer | 8 | Minimum password length |

---

### Logout

User logout component for ending sessions.

**Component ID:** `auth.logout`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| trigger | Input | Trigger | Triggers logout |
| success | Output | Trigger | Fires on successful logout |

---

### Session

Session management component for checking and managing user sessions.

**Component ID:** `auth.session`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| check | Input | Trigger | Check session validity |
| user | Output | Entity(User) | Current user if authenticated |
| valid | Output | Trigger | Fires if session is valid |
| invalid | Output | Trigger | Fires if session is invalid |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| refresh_threshold | Duration | 1h | Refresh session if older than this |

---

## 📊 Data Components

### Entity

Define a data model with fields and relationships. This is the core component for defining your database tables.

**Component ID:** `data.entity`

**Default Fields:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | UUID | Yes | Primary key (auto-generated) |
| created_at | DateTime | No | Creation timestamp |
| updated_at | DateTime | No | Last update timestamp |

**Custom Fields:**
You can add custom fields with the following types:
- String, Text
- Integer (i32), BigInt (i64)
- Float (f32), Double (f64)
- Boolean
- DateTime, Date, Time
- JSON, Bytes
- Reference (to another entity)

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| entity | Input | Entity | Receives entity for processing |
| on_create | Input | Trigger | Hook for creation events |
| on_update | Input | Trigger | Hook for update events |
| on_delete | Input | Trigger | Hook for delete events |
| entity | Output | Entity | Outputs entity data |
| list | Output | Array | Outputs list of entities |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| table_name | String | "" | Custom database table name |
| id_type | String | "uuid" | Primary key type (uuid, serial, cuid) |
| timestamps | Boolean | true | Auto-manage created/updated timestamps |
| soft_delete | Boolean | false | Use soft delete instead of hard delete |

**Visual Display:**
Entity nodes display their fields directly on the canvas:
```
┌─────────────────────────┐
│       📊 Todo           │
├─────────────────────────┤
│ 🔑 id              Uuid │
│ created_at     DateTime │
│ title           String  │
│ completed       Boolean │
└─────────────────────────┘
```

---

### Collection

A queryable collection of entities with filtering and pagination.

**Component ID:** `data.collection`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| query | Input | Query | Query parameters |
| items | Output | Array | Collection items |
| count | Output | Integer | Total count |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| default_limit | Integer | 20 | Default page size |
| max_limit | Integer | 100 | Maximum page size |

---

### Query

Build and execute database queries with conditions and joins.

**Component ID:** `data.query`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| execute | Input | Trigger | Execute the query |
| result | Output | Any | Query results |
| error | Output | Trigger | Fires on query error |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| timeout_ms | Integer | 30000 | Query timeout in milliseconds |

---

## 🔌 API Components

### REST Endpoint

Define a RESTful API endpoint with HTTP methods.

**Component ID:** `api.rest`

**Fields:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| path | String | Yes | URL path (e.g., "/todos") |
| method | String | Yes | HTTP method (GET, POST, PUT, DELETE) |

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| request | Input | Any | Incoming request data |
| request_body | Input | Any | Request body data |
| path_params | Input | Any | URL path parameters |
| query_params | Input | Any | URL query parameters |
| headers | Input | Any | Request headers |
| response | Output | Any | Response data |
| on_request | Output | Trigger | Fires when request received |
| error | Output | Trigger | Fires on error |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| auth_required | Boolean | false | Require authentication |
| rate_limit | Integer | 0 | Requests per minute (0 = unlimited) |
| cors_enabled | Boolean | true | Enable CORS |
| method | String | "GET" | HTTP method |
| response_type | String | "json" | Response content type |
| timeout_ms | Integer | 30000 | Request timeout |
| roles | String | "" | Required roles (comma-separated) |

---

### GraphQL

Define a GraphQL API with queries, mutations, and subscriptions.

**Component ID:** `api.graphql`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| query | Input | Any | GraphQL query |
| result | Output | Any | Query result |
| error | Output | Trigger | Fires on error |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| playground_enabled | Boolean | true | Enable GraphQL Playground |
| introspection | Boolean | true | Allow schema introspection |
| max_depth | Integer | 10 | Maximum query depth |

---

### WebSocket

WebSocket connection for real-time bidirectional communication.

**Component ID:** `api.websocket`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| connect | Input | Trigger | Client connected |
| message | Input | Any | Incoming message |
| send | Output | Any | Outgoing message |
| disconnect | Output | Trigger | Client disconnected |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| path | String | "/ws" | WebSocket endpoint path |
| heartbeat_interval | Integer | 30000 | Heartbeat interval (ms) |

---

## 💾 Storage Components

### Database

Database connection and configuration for data persistence.

**Component ID:** `storage.database`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| query | Input | Any | SQL query to execute |
| result | Output | Any | Query result |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| backend | String | "postgres" | Database type (postgres, mysql, sqlite) |
| connection_string | String | "" | Database connection URL |
| pool_size | Integer | 10 | Connection pool size |
| ssl_mode | String | "prefer" | SSL mode |

---

### Cache

In-memory or distributed caching for performance optimization.

**Component ID:** `storage.cache`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| get | Input | String | Key to retrieve |
| set | Input | Any | Value to store |
| value | Output | Any | Retrieved value |
| hit | Output | Trigger | Cache hit |
| miss | Output | Trigger | Cache miss |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| backend | String | "memory" | Cache backend (memory, redis) |
| ttl_seconds | Integer | 3600 | Default TTL in seconds |
| max_size | Integer | 1000 | Maximum cache entries |

---

### File Storage

File and blob storage for documents, images, and binary data.

**Component ID:** `storage.files`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| upload | Input | Bytes | File data to upload |
| download | Input | String | File ID to download |
| file | Output | Bytes | Downloaded file data |
| url | Output | String | File URL |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| backend | String | "local" | Storage backend (local, s3, gcs) |
| max_file_size | Integer | 10485760 | Max file size in bytes |
| allowed_types | String | "*" | Allowed MIME types |

---

## ⚙ Logic Components

### Validator

Validate data against configurable rules.

**Component ID:** `logic.validator`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| input | Input | Any | Data to validate |
| valid | Output | Any | Valid data (passed through) |
| invalid | Output | Any | Invalid data with errors |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| rules | String | "" | Validation rules (JSON) |
| fail_fast | Boolean | false | Stop on first error |

---

### Transformer

Transform and map data between formats.

**Component ID:** `logic.transformer`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| input | Input | Any | Data to transform |
| output | Output | Any | Transformed data |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| mapping | String | "" | Field mapping configuration |

---

### Condition

Conditional branching based on an expression.

**Component ID:** `logic.condition`

**Ports:**
| Port | Direction | Type | Description |
|------|-----------|------|-------------|
| input | Input | Any | Data to evaluate |
| true | Output | Any | Output if condition is true |
| false | Output | Any | Output if condition is false |

**Configuration:**
| Option | Type | Default | Description |
|--------|------|---------|-------------|
| expression | String | "" | Condition expression |

---

## Connection Types

When connecting components, different connection types are created based on the components involved:

| Connection Type | Visual | Description |
|-----------------|--------|-------------|
| Data Flow | Solid line | Data passes from source to target |
| Trigger | Dashed line | Event triggers an action |
| Relationship | Line with markers | Entity relationship (1:1, 1:N, N:M) |
| Dependency | Dotted line | Target depends on source |

---

## Field Types Reference

Available data types for entity fields:

| Type | Rust Type | Description |
|------|-----------|-------------|
| String | `String` | Variable-length text |
| Text | `String` | Long-form text content |
| Integer | `i32` | 32-bit signed integer |
| BigInt | `i64` | 64-bit signed integer |
| Float | `f32` | 32-bit floating point |
| Double | `f64` | 64-bit floating point |
| Boolean | `bool` | True/false value |
| UUID | `uuid::Uuid` | Universally unique identifier |
| DateTime | `chrono::DateTime<Utc>` | Date and time with timezone |
| Date | `chrono::NaiveDate` | Date without time |
| Time | `chrono::NaiveTime` | Time without date |
| Bytes | `Vec<u8>` | Binary data |
| JSON | `serde_json::Value` | JSON data |

---

## Best Practices

### Entity Design

1. **Always use UUID for IDs** - More portable and secure than auto-increment
2. **Enable timestamps** - Track when records are created/updated
3. **Consider soft delete** - For data that might need recovery
4. **Use meaningful names** - Entity and field names should be self-documenting

### API Design

1. **Use RESTful conventions** - Standard HTTP methods and status codes
2. **Enable CORS when needed** - For cross-origin requests
3. **Set appropriate rate limits** - Protect against abuse
4. **Require auth for sensitive endpoints** - Secure your data

### Connection Patterns

1. **Entity → REST Endpoint** - Expose entity via API
2. **Entity → Database** - Persist entity data
3. **Login → Session** - Manage user authentication
4. **REST Endpoint → Validator → Entity** - Validate before saving

---

## Adding Custom Components

Custom components can be created by implementing the component traits. See the [Architecture](./architecture.md) documentation for details on extending the component system.