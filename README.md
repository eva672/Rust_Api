# Rust API with Keycloak and PostgreSQL

A production-ready Rust API application featuring PostgreSQL database integration, Keycloak authentication, and Kubernetes deployment on a local K3s cluster.

## 🚀 Features

- **Rust Web API**: Built with Warp framework for high performance
- **PostgreSQL Database**: Managed by CloudNativePG (CNPG) operator
- **Keycloak Authentication**: JWT token verification and user management
- **Kubernetes Deployment**: Complete K8s manifests for production deployment
- **Professional Logging**: Structured logging with emojis and detailed output
- **Automatic Migrations**: Database schema management with SQLx
- **Local Development**: Full local development environment with Docker
- **Infrastructure as Code**: Ansible playbooks for VM provisioning

## 📋 Project Structure

```
rust_api/
├── app/                    # Rust application source code
│   ├── src/               # Source code
│   │   ├── main.rs        # Application entry point
│   │   ├── config.rs      # Configuration management
│   │   ├── db.rs          # Database connection and migrations
│   │   ├── jwt.rs         # JWT verification and Keycloak integration
│   │   ├── error.rs       # Custom error types
│   │   ├── handlers/      # API route handlers
│   │   ├── middleware/    # Authentication middleware
│   │   └── models/        # Data models
│   ├── migrations/        # Database migration scripts
│   ├── Cargo.toml         # Rust dependencies
│   └── Dockerfile         # Docker build configuration
├── deploy/                # Kubernetes deployment manifests
│   ├── app/              # Application deployment
│   ├── postgres/         # CNPG PostgreSQL deployment
│   └── keycloak/         # Keycloak deployment
├── docs/                 # Documentation
│   ├── api-documentation.md
│   ├── deployment-guide.md
│   ├── development-progress.md
│   └── runbooks/
├── infra/                # Infrastructure as Code
│   └── ansible/          # Ansible playbooks and roles
└── scripts/              # Utility scripts
    ├── setup.sh          # Complete infrastructure setup
    └── image-list.txt    # Container images to pre-pull
```

## 🛠️ Quick Start

### Prerequisites

- **Multipass**: For VM management
- **Ansible**: For infrastructure provisioning
- **Docker**: For container builds
- **kubectl**: For Kubernetes management
- **Rust**: For local development

### 1. Infrastructure Setup

```bash
# Clone the repository
git clone <repository-url>
cd rust_api

# Run complete infrastructure setup
./scripts/setup.sh
```

This will:

- Create a Multipass VM with K3s cluster
- Set up local Docker registry
- Pre-pull all required container images
- Configure DNS resolution

### 2. Deploy Database

```bash
# Deploy CNPG operator
kubectl apply -f deploy/postgres/cnpg-operator.yaml

# Deploy PostgreSQL cluster
kubectl apply -f deploy/postgres/postgres-cluster.yaml

# Verify deployment
kubectl get pods -n cnpg
```

### 3. Build and Deploy Application

```bash
# Build Docker image
cd app
docker build -t registry.local:5000/rust_api:latest .

# Push to local registry
docker push registry.local:5000/rust_api:latest

# Deploy application
kubectl apply -f ../deploy/app/

# Verify deployment
kubectl get pods -n myapp
```

### 4. Test the API

```bash
# Test health endpoint
curl http://api.local/health

# Test tasks endpoint
curl http://api.local/api/tasks

# Create a task
curl -X POST http://api.local/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust",
    "description": "Complete Rust API development",
    "completed": false
  }'
```

## 🔧 Local Development

### Environment Setup

```bash
# Navigate to app directory
cd app

# Set environment variables
export DATABASE_URL="postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb"
export RUST_LOG=info

# Run the application
cargo run
```

### Database Connection

The application uses the following DATABASE_URL format:

```
postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb
```

### API Endpoints

- **GET** `/health` - Health check
- **GET** `/api/tasks` - List all tasks
- **POST** `/api/tasks` - Create new task

## 📊 Database Schema

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

## 🔐 Authentication

The application integrates with Keycloak for JWT authentication:

- **JWKS Endpoint**: Automatic key rotation support
- **Token Introspection**: Optional online validation
- **Claims Processing**: User information extraction
- **Middleware**: Authentication middleware for protected routes

## 📈 Monitoring and Logging

### Professional Logging

The application features comprehensive logging:

```
[INFO] 🚀 Starting Rust API application...
[INFO] 🔌 Connecting to PostgreSQL database...
[INFO] ✅ Successfully connected to PostgreSQL database
[INFO] 🔄 Starting database migrations...
[INFO] 📋 Creating users table...
[INFO] ✅ Users table created successfully
[INFO] 📊 Creating database indexes...
[INFO] ✅ All database indexes created successfully
[INFO] 🎉 Database migrations completed successfully
[INFO] 🚀 Server running on http://0.0.0.0:3000
```

### Health Checks

- Application health endpoint
- Database connection verification
- CNPG cluster status monitoring

## 🚀 Production Deployment

### Kubernetes Manifests

All deployment manifests are included:

- **Application**: Deployment, Service, Ingress
- **Database**: CNPG operator and cluster
- **Authentication**: Keycloak deployment
- **Configuration**: Secrets and ConfigMaps

### Environment Configuration

```yaml
# Application Configuration
APP_HOST: "0.0.0.0"
APP_PORT: "3000"
RUST_LOG: "info"

# Database Configuration
DATABASE_URL: "postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb"

# Keycloak Configuration
KEYCLOAK_BASE_URL: "http://keycloak.keycloak.svc.cluster.local:8080"
KEYCLOAK_REALM: "rust-api-realm"
KEYCLOAK_CLIENT_ID: "rust-api-client"
KEYCLOAK_CLIENT_SECRET: "your-client-secret"
```

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

## 🔧 Troubleshooting

### Common Issues

1. **Database Connection Failed**

   - Verify CNPG cluster is running
   - Check DATABASE_URL format
   - Ensure network connectivity

2. **Image Pull Failed**

   - Verify local registry is running
   - Check image exists in registry
   - Rebuild and push image

3. **Pod CrashLoopBackOff**
   - Check pod logs for errors
   - Verify environment variables
   - Check database connectivity

### Debug Commands

```bash
# Check all resources
kubectl get all -n myapp
kubectl get all -n cnpg

# Check logs
kubectl logs -f <pod-name> -n myapp

# Check events
kubectl get events -n myapp --sort-by='.lastTimestamp'
```

## 📚 Documentation

- **[API Documentation](docs/api-documentation.md)** - Complete API reference
- **[Deployment Guide](docs/deployment-guide.md)** - Step-by-step deployment instructions
- **[Development Progress](docs/development-progress.md)** - Complete development history
- **[Runbooks](docs/runbooks/)** - Operational procedures

## 🏆 Achievements

This project demonstrates:

- **Full-stack development** with Rust, PostgreSQL, and Kubernetes
- **Infrastructure as Code** with Ansible and Kubernetes manifests
- **Professional software engineering** practices
- **Cloud-native application** development
- **Microservices architecture** with proper separation of concerns
- **Production-ready** application with comprehensive logging and error handling

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🎯 Roadmap

- [ ] Add comprehensive unit and integration tests
- [ ] Implement OpenAPI/Swagger documentation
- [ ] Add Prometheus metrics and Grafana dashboards
- [ ] Implement Redis caching
- [ ] Add rate limiting for API endpoints
- [ ] Set up CI/CD pipeline
- [ ] Add comprehensive monitoring and alerting

## 📞 Support

For support and questions:

- Create an issue in the repository
- Check the documentation in the `docs/` directory
- Review the troubleshooting section

---

**Built with ❤️ using Rust, PostgreSQL, and Kubernetes**

