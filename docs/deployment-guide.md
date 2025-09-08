# Deployment Guide

## 🚀 Complete Deployment Guide

This guide covers the complete deployment of the Rust API application with PostgreSQL (CNPG) and Keycloak on a local Kubernetes cluster.

## 📋 Prerequisites

### Infrastructure Requirements

- **Multipass VM**: `k3s-host-1` with K3s cluster
- **Local Registry**: `registry.local:5000` with pre-pulled images
- **DNS Resolution**: `registry.local` hostname configured

### Required Images

All images should be pre-pulled in the local registry:

```
quay.io/keycloak/keycloak:24.0.5
ghcr.io/cloudnative-pg/cloudnative-pg:1.23.0
ghcr.io/cloudnative-pg/postgresql:16.2
gitea/gitea:1.22.3
cr.l5d.io/linkerd/controller:edge-25.8.5
cr.l5d.io/linkerd/proxy:edge-25.8.5
nginx:1.25.5
alpine:3.19
busybox:1.36.0
```

## 🗄️ Step 1: Deploy PostgreSQL (CNPG)

### 1.1 Deploy CNPG Operator

```bash
# Apply CNPG operator
kubectl apply -f deploy/postgres/cnpg-operator.yaml

# Verify operator deployment
kubectl get pods -n cnpg-system
```

**Expected Output:**

```
NAME                                       READY   STATUS    RESTARTS   AGE
cnpg-controller-manager-7976cfb87c-lsqrw   1/1     Running   1          5h1m
```

### 1.2 Create PostgreSQL Cluster

```bash
# Apply PostgreSQL cluster
kubectl apply -f deploy/postgres/postgres-cluster.yaml

# Verify cluster deployment
kubectl get pods -n cnpg
kubectl get clusters -n cnpg
```

**Expected Output:**

```
NAME            READY   STATUS    RESTARTS   AGE
rust-api-db-1   1/1     Running   1          4h55m

NAME           AGE   INSTANCES   READY   STATUS                     PRIMARY
rust-api-db    5h    1           1       Cluster in healthy state   rust-api-db-1
```

### 1.3 Verify Database Connection

```bash
# Connect to database
kubectl exec -it rust-api-db-1 -n cnpg -- psql -U postgres -d appdb

# Check if appdb database exists
\l

# Exit psql
\q
```

## 🔐 Step 2: Deploy Keycloak (Optional)

### 2.1 Deploy Keycloak

```bash
# Apply Keycloak deployment
kubectl apply -f deploy/keycloak/

# Verify Keycloak deployment
kubectl get pods -n keycloak
kubectl get services -n keycloak
```

### 2.2 Configure Keycloak

1. Access Keycloak admin console
2. Create realm: `rust-api-realm`
3. Create client: `rust-api-client`
4. Configure client secret
5. Set up users and roles

## 🚀 Step 3: Build and Deploy Rust API

### 3.1 Build Docker Image

```bash
# Navigate to app directory
cd /home/eva/Projects/rust_api/app

# Build Docker image
docker build -t registry.local:5000/rust_api:latest .

# Push to local registry
docker push registry.local:5000/rust_api:latest
```

### 3.2 Deploy Application

```bash
# Apply application deployment
kubectl apply -f deploy/app/

# Verify deployment
kubectl get pods -n myapp
kubectl get services -n myapp
kubectl get ingress -n myapp
```

**Expected Output:**

```
NAME                        READY   STATUS    RESTARTS   AGE
rust-api-7d4b8c9f6-abc123   1/1     Running   0          2m

NAME           TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)   AGE
rust-api-svc   ClusterIP   10.43.123.45    <none>        3000/TCP  2m

NAME                    CLASS   HOSTS              ADDRESS   PORTS   AGE
rust-api-ingress        nginx   api.local          80        2m
```

## 🔍 Step 4: Verification

### 4.1 Check Application Logs

```bash
# Get pod name
kubectl get pods -n myapp

# Check logs
kubectl logs -f <pod-name> -n myapp
```

**Expected Logs:**

```
[INFO] 🚀 Starting Rust API application...
[INFO] 🔌 Connecting to PostgreSQL database...
[INFO] ✅ Successfully connected to PostgreSQL database
[INFO] 🔄 Starting database migrations...
[INFO] 📋 Creating users table...
[INFO] ✅ Users table created successfully
[INFO] 📋 Creating tasks table...
[INFO] ✅ Tasks table created successfully
[INFO] 📊 Creating database indexes...
[INFO] ✅ All database indexes created successfully
[INFO] 🎉 Database migrations completed successfully
[INFO] 🔍 Verifying database connection...
[INFO] ✅ Database connection verified - 2 tables found in public schema
[INFO] 🚀 Server running on http://0.0.0.0:3000
```

### 4.2 Test API Endpoints

```bash
# Test health endpoint
curl http://api.local/health

# Test tasks endpoint
curl http://api.local/api/tasks

# Create a task
curl -X POST http://api.local/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test Task",
    "description": "Created via API",
    "completed": false
  }'
```

### 4.3 Verify Database

```bash
# Connect to database
kubectl exec -it rust-api-db-1 -n cnpg -- psql -U postgres -d appdb

# Check tables
\dt

# Check data
SELECT * FROM tasks;
SELECT * FROM users;

# Exit
\q
```

## 🔧 Configuration Files

### Application Configuration

**File**: `deploy/app/app-db-secret.yaml`

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: app-db-secret
  namespace: myapp
type: Opaque
stringData:
  DATABASE_URL: "postgres://postgres:postgres@rust-api-db-rw.cnpg.svc.cluster.local:5432/appdb"
  KEYCLOAK_BASE_URL: "http://keycloak.keycloak.svc.cluster.local:8080"
  KEYCLOAK_REALM: "rust-api-realm"
  KEYCLOAK_CLIENT_ID: "rust-api-client"
  KEYCLOAK_CLIENT_SECRET: "your-client-secret"
  RUST_LOG: "info"
```

### Database Configuration

**File**: `deploy/postgres/postgres-cluster.yaml`

```yaml
apiVersion: postgresql.cnpg.io/v1
kind: Cluster
metadata:
  name: rust-api-db
  namespace: cnpg
spec:
  instances: 1
  storage:
    size: 1Gi
  superuserSecret:
    name: rust-api-db-superuser
  enableSuperuserAccess: true
  bootstrap:
    initdb:
      dataChecksums: true
      database: appdb
```

### Application Deployment

**File**: `deploy/app/rust-api-deployment.yaml`

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-api
  namespace: myapp
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rust-api
  template:
    metadata:
      labels:
        app: rust-api
    spec:
      containers:
        - name: rust-api
          image: registry.local:5000/rust_api:latest
          ports:
            - containerPort: 3000
          envFrom:
            - secretRef:
                name: app-db-secret
          env:
            - name: APP_HOST
              value: "0.0.0.0"
            - name: APP_PORT
              value: "3000"
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Database Connection Failed

**Error**: `failed to lookup address information: Temporary failure in name resolution`

**Solution**:

- Verify CNPG cluster is running: `kubectl get pods -n cnpg`
- Check service exists: `kubectl get svc -n cnpg`
- Verify DATABASE_URL format

#### 2. Image Pull Failed

**Error**: `Failed to pull image "registry.local:5000/rust_api:latest"`

**Solution**:

- Verify local registry is running
- Check image exists: `curl -s http://registry.local:5000/v2/_catalog | jq .`
- Rebuild and push image

#### 3. Pod CrashLoopBackOff

**Error**: Pod keeps restarting

**Solution**:

- Check pod logs: `kubectl logs <pod-name> -n myapp`
- Verify environment variables
- Check database connectivity

#### 4. Ingress Not Working

**Error**: Cannot access API via ingress

**Solution**:

- Check ingress controller: `kubectl get pods -n ingress-nginx`
- Verify ingress configuration
- Check DNS resolution for `api.local`

### Debug Commands

```bash
# Check all resources
kubectl get all -n myapp
kubectl get all -n cnpg
kubectl get all -n keycloak

# Check events
kubectl get events -n myapp --sort-by='.lastTimestamp'

# Describe resources
kubectl describe pod <pod-name> -n myapp
kubectl describe service rust-api-svc -n myapp

# Check logs
kubectl logs -f <pod-name> -n myapp
kubectl logs -f rust-api-db-1 -n cnpg
```

## 📊 Monitoring

### Health Checks

```bash
# Application health
curl http://api.local/health

# Database health
kubectl exec -it rust-api-db-1 -n cnpg -- pg_isready -U postgres

# CNPG cluster status
kubectl get clusters -n cnpg
```

### Resource Usage

```bash
# Check resource usage
kubectl top pods -n myapp
kubectl top pods -n cnpg

# Check node resources
kubectl top nodes
```

## 🔄 Updates and Maintenance

### Application Updates

```bash
# Build new image
docker build -t registry.local:5000/rust_api:v2.0.0 .

# Push new image
docker push registry.local:5000/rust_api:v2.0.0

# Update deployment
kubectl set image deployment/rust-api rust-api=registry.local:5000/rust_api:v2.0.0 -n myapp

# Verify rollout
kubectl rollout status deployment/rust-api -n myapp
```

### Database Maintenance

```bash
# Backup database
kubectl exec -it rust-api-db-1 -n cnpg -- pg_dump -U postgres appdb > backup.sql

# Restore database
kubectl exec -i rust-api-db-1 -n cnpg -- psql -U postgres appdb < backup.sql
```

## 🎯 Production Considerations

### Security

- Use proper secrets management
- Enable TLS/SSL for all communications
- Implement network policies
- Regular security updates

### Scalability

- Configure horizontal pod autoscaling
- Use multiple database replicas
- Implement load balancing
- Monitor resource usage

### Backup and Recovery

- Regular database backups
- Application state persistence
- Disaster recovery procedures
- Monitoring and alerting

## 📝 Cleanup

### Remove Application

```bash
# Delete application
kubectl delete -f deploy/app/

# Delete namespace
kubectl delete namespace myapp
```

### Remove Database

```bash
# Delete PostgreSQL cluster
kubectl delete -f deploy/postgres/postgres-cluster.yaml

# Delete CNPG operator
kubectl delete -f deploy/postgres/cnpg-operator.yaml

# Delete namespace
kubectl delete namespace cnpg
```

### Remove Keycloak

```bash
# Delete Keycloak
kubectl delete -f deploy/keycloak/

# Delete namespace
kubectl delete namespace keycloak
```

## 🏆 Success Criteria

Deployment is successful when:

1. ✅ All pods are running and healthy
2. ✅ Database migrations completed successfully
3. ✅ API endpoints respond correctly
4. ✅ Database contains expected tables and data
5. ✅ Logs show no errors
6. ✅ Health checks pass
7. ✅ Ingress is accessible from outside cluster

The application is now ready for production use! 🚀

