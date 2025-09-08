# Rust API Documentation

## 🚀 API Overview

The Rust API is a task management application built with Warp framework, featuring PostgreSQL database integration and Keycloak authentication. It provides RESTful endpoints for task management with professional logging and error handling.

## 📋 Base Configuration

- **Base URL**: `http://localhost:3000` (local development)
- **Framework**: Warp 0.3
- **Database**: PostgreSQL with SQLx
- **Authentication**: Keycloak JWT integration
- **Logging**: Structured logging with emojis

## 🔧 Environment Variables

```bash
# Application Configuration
APP_HOST=0.0.0.0
APP_PORT=3000
RUST_LOG=info

# Database Configuration
DATABASE_URL=postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb

# Keycloak Configuration
KEYCLOAK_BASE_URL=http://keycloak.local:8080
KEYCLOAK_REALM=rust-api-realm
KEYCLOAK_CLIENT_ID=rust-api-client
KEYCLOAK_CLIENT_SECRET=your-client-secret-here
```

## 📡 API Endpoints

### Health Check

**GET** `/health`

Check if the API is running and healthy.

**Response:**

```json
{
  "status": "ok"
}
```

**Example:**

```bash
curl http://localhost:3000/health
```

### Task Management

#### List All Tasks

**GET** `/api/tasks`

Retrieve all tasks from the database.

**Response:**

```json
[
  {
    "id": "55b83e42-cf6b-4bc3-9339-dfb741d3d433",
    "user_id": "37f23f7c-ee40-40ea-b001-5cac1de9dd83",
    "title": "Test Task",
    "description": "This is a test task created via API",
    "completed": false,
    "created_at": "2025-09-07T19:51:06.123416878+00:00",
    "updated_at": "2025-09-07T19:51:06.123416878+00:00"
  }
]
```

**Example:**

```bash
curl http://localhost:3000/api/tasks
```

#### Create New Task

**POST** `/api/tasks`

Create a new task in the database.

**Request Body:**

```json
{
  "title": "New Task",
  "description": "Task description (optional)",
  "completed": false
}
```

**Response:**

```json
{
  "id": "6ea4e755-636e-4965-9529-119e779776dc",
  "user_id": "20bf517a-28bc-4918-ba39-0cc3a4c17697",
  "title": "New Task",
  "description": "Task description (optional)",
  "completed": false,
  "created_at": "2025-09-07T19:51:10.082091891+00:00",
  "updated_at": "2025-09-07T19:51:10.082091891+00:00"
}
```

**Example:**

```bash
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust",
    "description": "Complete Rust API development",
    "completed": false
  }'
```

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Tasks Table

```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Indexes

```sql
-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id);
CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
```

## 🔐 Authentication (Keycloak Integration)

### JWT Token Verification

The application supports JWT token verification with Keycloak:

- **JWKS Endpoint**: `{KEYCLOAK_BASE_URL}/realms/{REALM}/protocol/openid-connect/certs`
- **Token Introspection**: Optional online validation
- **Claims Processing**: User information extraction from JWT

### Authentication Middleware

```rust
pub struct AuthUser {
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
}
```

### Protected Endpoints

Currently, all endpoints are accessible without authentication. To enable authentication:

1. Uncomment authentication middleware in route handlers
2. Configure Keycloak realm and client
3. Add JWT token validation to protected routes

## 📊 Logging

### Log Levels

- **INFO**: Application startup, database operations, API requests
- **DEBUG**: Detailed database queries, connection details
- **ERROR**: Connection failures, validation errors

### Log Format

```
[2025-09-07T20:54:33Z INFO  rust_api] 🚀 Starting Rust API application...
[2025-09-07T20:54:33Z INFO  rust_api::db] 🔌 Connecting to PostgreSQL database...
[2025-09-07T20:54:33Z INFO  rust_api::db] 🔍 Database URL: postgres:***:***@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb
[2025-09-07T20:54:33Z INFO  rust_api::db] ✅ Successfully connected to PostgreSQL database
[2025-09-07T20:54:33Z INFO  rust_api::db] 🔄 Starting database migrations...
[2025-09-07T20:54:33Z INFO  rust_api::db] 📋 Creating users table...
[2025-09-07T20:54:33Z INFO  rust_api::db] ✅ Users table created successfully
[2025-09-07T20:54:33Z INFO  rust_api::db] 📋 Creating tasks table...
[2025-09-07T20:54:33Z INFO  rust_api::db] ✅ Tasks table created successfully
[2025-09-07T20:54:33Z INFO  rust_api::db] 📊 Creating database indexes...
[2025-09-07T20:54:33Z INFO  rust_api::db] ✅ All database indexes created successfully
[2025-09-07T20:54:33Z INFO  rust_api::db] 🎉 Database migrations completed successfully
[2025-09-07T20:54:33Z INFO  rust_api::db] 🔍 Verifying database connection...
[2025-09-07T20:54:33Z INFO  rust_api::db] ✅ Database connection verified - 2 tables found in public schema
[2025-09-07T20:54:33Z INFO  warp::server] Server::run; addr=0.0.0.0:3000
[2025-09-07T20:54:33Z INFO  warp::server] listening on http://0.0.0.0:3000
```

## 🚀 Running the Application

### Local Development

```bash
# Navigate to app directory
cd /home/eva/Projects/rust_api/app

# Set environment variables
export DATABASE_URL="postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb"
export RUST_LOG=info

# Run the application
cargo run
```

### Docker

```bash
# Build image
docker build -t registry.local:5000/rust_api:latest .

# Run container
docker run -p 3000:3000 \
  -e DATABASE_URL="postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb" \
  -e RUST_LOG=info \
  registry.local:5000/rust_api:latest
```

### Kubernetes

```bash
# Deploy to Kubernetes
kubectl apply -f deploy/app/

# Check deployment status
kubectl get pods -n myapp
kubectl get services -n myapp
```

## 🔧 Error Handling

### Common Error Responses

#### Database Connection Error

```json
{
  "error": "Database connection failed: error communicating with database: failed to lookup address information: Temporary failure in name resolution"
}
```

#### Validation Error

```json
{
  "error": "Invalid request: missing required field 'title'"
}
```

#### Internal Server Error

```json
{
  "error": "Internal server error: failed to create task"
}
```

## 📈 Performance

### Connection Pool Configuration

```rust
let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(1)
    .acquire_timeout(std::time::Duration::from_secs(30))
    .idle_timeout(std::time::Duration::from_secs(600))
    .max_lifetime(std::time::Duration::from_secs(1800))
    .connect(&database_url)
    .await?;
```

### Database Indexes

- **User ID Index**: Fast task lookup by user
- **Completion Status Index**: Efficient filtering by completion status
- **Email Index**: Quick user lookup by email
- **Created At Index**: Optimized date-based queries

## 🧪 Testing

### Manual Testing

```bash
# Health check
curl http://localhost:3000/health

# List tasks
curl http://localhost:3000/api/tasks

# Create task
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Task", "description": "Test Description", "completed": false}'
```

### Database Testing

```bash
# Connect to database
kubectl exec -it rust-api-db-1 -n cnpg -- psql -U postgres -d appdb

# Check tables
\dt

# Check data
SELECT * FROM tasks;
SELECT * FROM users;
```

## 🔄 Database Migrations

The application automatically runs database migrations on startup:

1. **Table Creation**: Creates users and tasks tables if they don't exist
2. **Index Creation**: Adds performance indexes
3. **Verification**: Confirms database connection and schema

### Migration Files

- **Location**: `app/migrations/20250107000001_create_users_and_tasks_tables/`
- **Up Migration**: `up.sql` - Creates tables and indexes
- **Down Migration**: `down.sql` - Drops tables

## 📝 Development Notes

### Code Structure

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── db.rs            # Database connection and migrations
├── jwt.rs           # JWT verification and Keycloak integration
├── error.rs         # Custom error types
├── handlers/        # API route handlers
│   ├── mod.rs
│   └── task.rs
├── middleware/      # Authentication middleware
│   ├── mod.rs
│   └── auth.rs
└── models/          # Data models
    ├── mod.rs
    ├── task.rs
    └── user.rs
```

### Key Features

- **Professional Logging**: Comprehensive logging with emojis and structured output
- **Error Handling**: Detailed error messages with helpful suggestions
- **Database Integration**: Automatic migrations and connection pooling
- **Keycloak Integration**: JWT verification and token introspection
- **Production Ready**: Kubernetes deployment manifests and configuration

## 🎯 Future Enhancements

1. **Authentication**: Enable JWT authentication on protected routes
2. **API Documentation**: Generate OpenAPI/Swagger documentation
3. **Testing**: Add comprehensive unit and integration tests
4. **Monitoring**: Add Prometheus metrics and Grafana dashboards
5. **Caching**: Implement Redis caching for improved performance
6. **Rate Limiting**: Add rate limiting for API endpoints
7. **Validation**: Enhanced request validation and sanitization

