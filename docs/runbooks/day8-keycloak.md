# Day 8: Bow Before Keycloak - Implementation Guide

## 🎯 Overview

This guide covers the complete implementation of Keycloak authentication for the Rust API application, including deployment, configuration, and JWT integration.

## ✅ Current Status Assessment

### What We Have Implemented:

#### 1. ✅ Keycloak Deployment Manifests

- **Deployment**: `deploy/keycloak/keycloak-deployment.yaml`
- **Service**: ClusterIP service on port 80/443
- **Ingress**: `keycloak.local` hostname configuration
- **Secrets**: Admin credentials and database configuration
- **Database**: Separate CNPG cluster for Keycloak

#### 2. ✅ JWT Integration in Rust App

- **JWT Verification**: Complete JWT verification with JWKS
- **Claims Processing**: User information extraction
- **Token Introspection**: Optional online validation
- **Authentication Middleware**: Ready for protected routes
- **Configuration**: Keycloak URLs and client settings

#### 3. ✅ Database Integration

- **Separate Database**: Keycloak has its own PostgreSQL cluster
- **Connection Configuration**: Proper database URL and credentials
- **Storage**: 1Gi persistent storage for Keycloak data

### What Needs to be Completed:

#### 1. 🔄 Deploy Keycloak Infrastructure

#### 2. 🔄 Configure Keycloak Realm and Client

#### 3. 🔄 Create Test User

#### 4. 🔄 Enable JWT Authentication in App

#### 5. 🔄 Test Complete Authentication Flow

## 🚀 Step-by-Step Implementation

### Step 1: Deploy Keycloak Infrastructure

#### 1.1 Create Keycloak Namespace

```bash
kubectl create namespace keycloak
```

#### 1.2 Deploy Keycloak Database

```bash
# Deploy Keycloak database superuser secret
kubectl apply -f deploy/keycloak/keycloak-db-superuser.yaml

# Deploy Keycloak database cluster
kubectl apply -f deploy/keycloak/keycloak-db-cluster.yaml

# Verify database deployment
kubectl get pods -n keycloak
kubectl get clusters -n keycloak
```

#### 1.3 Deploy Keycloak Application

```bash
# Deploy Keycloak secrets
kubectl apply -f deploy/keycloak/keycloak-secrets.yaml

# Deploy Keycloak application
kubectl apply -f deploy/keycloak/keycloak-deployment.yaml

# Deploy Keycloak ingress
kubectl apply -f deploy/keycloak/keycloak-ingress.yaml

# Verify deployment
kubectl get pods -n keycloak
kubectl get services -n keycloak
kubectl get ingress -n keycloak
```

### Step 2: Configure Keycloak

#### 2.1 Access Keycloak Admin Console

```bash
# Add keycloak.local to hosts (if not already done)
echo "127.0.0.1 keycloak.local" | sudo tee -a /etc/hosts

# Access Keycloak admin console
open http://keycloak.local
# Or: curl http://keycloak.local
```

**Login Credentials:**

- **Username**: `admin`
- **Password**: `supersecretadminpassword`

#### 2.2 Create Realm

1. Click "Create Realm"
2. **Realm name**: `rust-api-realm`
3. **Enabled**: `ON`
4. Click "Create"

#### 2.3 Create Client

1. Go to "Clients" → "Create client"
2. **Client type**: `OpenID Connect`
3. **Client ID**: `rust-api-client`
4. **Client authentication**: `ON`
5. **Authorization**: `OFF`
6. **Authentication flow**: `Standard flow`
7. **Valid redirect URIs**:
   - `http://api.local/*`
   - `http://localhost:3000/*`
8. **Web origins**:
   - `http://api.local`
   - `http://localhost:3000`
9. Click "Save"

#### 2.4 Configure Client Credentials

1. Go to "Clients" → "rust-api-client" → "Credentials"
2. Copy the **Client Secret**
3. Update the application secret with this value

#### 2.5 Create Test User

1. Go to "Users" → "Create new user"
2. **Username**: `testuser`
3. **Email**: `testuser@example.com`
4. **First name**: `Test`
5. **Last name**: `User`
6. **Email verified**: `ON`
7. **Enabled**: `ON`
8. Click "Create"

#### 2.6 Set User Password

1. Go to "Users" → "testuser" → "Credentials"
2. Click "Set password"
3. **Password**: `testpassword123`
4. **Temporary**: `OFF`
5. Click "Save"

### Step 3: Update Application Configuration

#### 3.1 Update Application Secrets

Update `deploy/app/app-db-secret.yaml` with the actual Keycloak client secret:

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
  KEYCLOAK_CLIENT_SECRET: "YOUR_ACTUAL_CLIENT_SECRET_HERE"
  RUST_LOG: "info"
```

#### 3.2 Enable Authentication in Application

Update the application to use authentication middleware on protected routes.

### Step 4: Test Authentication Flow

#### 4.1 Get Access Token

```bash
# Get access token using client credentials
curl -X POST http://keycloak.local/realms/rust-api-realm/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=rust-api-client" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "username=testuser" \
  -d "password=testpassword123"
```

**Expected Response:**

```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 300,
  "refresh_expires_in": 1800,
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "not-before-policy": 0,
  "session_state": "session-id",
  "scope": "profile email"
}
```

#### 4.2 Test Protected Endpoints

```bash
# Test with valid token
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  http://api.local/api/tasks

# Test without token (should fail)
curl http://api.local/api/tasks
```

## 🔧 Configuration Files

### Keycloak Deployment

**File**: `deploy/keycloak/keycloak-deployment.yaml`

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: keycloak
  namespace: keycloak
spec:
  replicas: 1
  selector:
    matchLabels:
      app: keycloak
  template:
    metadata:
      labels:
        app: keycloak
    spec:
      containers:
        - name: keycloak
          image: quay.io/keycloak/keycloak:24.0.5
          envFrom:
            - secretRef:
                name: keycloak-secrets
          ports:
            - containerPort: 8080
          args: ["start-dev"]
```

### Keycloak Ingress

**File**: `deploy/keycloak/keycloak-ingress.yaml`

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: keycloak-ingress
  namespace: keycloak
spec:
  ingressClassName: traefik
  rules:
    - host: keycloak.local
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: keycloak-service
                port:
                  number: 80
```

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
  KEYCLOAK_CLIENT_SECRET: "your-actual-client-secret"
  RUST_LOG: "info"
```

## 🧪 Testing Commands

### Health Checks

```bash
# Check Keycloak health
curl http://keycloak.local/health

# Check application health
curl http://api.local/health
```

### Authentication Tests

```bash
# Get token
TOKEN=$(curl -s -X POST http://keycloak.local/realms/rust-api-realm/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=rust-api-client" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "username=testuser" \
  -d "password=testpassword123" | jq -r '.access_token')

# Test protected endpoint
curl -H "Authorization: Bearer $TOKEN" http://api.local/api/tasks

# Test without token (should fail)
curl http://api.local/api/tasks
```

### Database Verification

```bash
# Check Keycloak database
kubectl exec -it keycloak-db-1 -n keycloak -- psql -U postgres -d keycloak

# Check application database
kubectl exec -it rust-api-db-1 -n cnpg -- psql -U postgres -d appdb
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Keycloak Not Accessible

**Error**: Cannot access `keycloak.local`

**Solution**:

- Check ingress: `kubectl get ingress -n keycloak`
- Verify DNS: `nslookup keycloak.local`
- Check pods: `kubectl get pods -n keycloak`

#### 2. Database Connection Failed

**Error**: Keycloak cannot connect to database

**Solution**:

- Check database pod: `kubectl get pods -n keycloak`
- Verify database URL in secrets
- Check database logs: `kubectl logs keycloak-db-1 -n keycloak`

#### 3. JWT Verification Failed

**Error**: Token verification fails in application

**Solution**:

- Check JWKS endpoint: `curl http://keycloak.local/realms/rust-api-realm/protocol/openid-connect/certs`
- Verify client configuration
- Check application logs for JWT errors

#### 4. Client Secret Mismatch

**Error**: Invalid client credentials

**Solution**:

- Verify client secret in Keycloak admin console
- Update application secret with correct value
- Restart application pods

### Debug Commands

```bash
# Check all Keycloak resources
kubectl get all -n keycloak

# Check Keycloak logs
kubectl logs -f deployment/keycloak -n keycloak

# Check application logs
kubectl logs -f deployment/rust-api -n myapp

# Check ingress
kubectl describe ingress keycloak-ingress -n keycloak
```

## ✅ Success Criteria

Day 8 is complete when:

1. ✅ Keycloak is deployed and accessible at `keycloak.local`
2. ✅ Keycloak realm `rust-api-realm` is created
3. ✅ Client `rust-api-client` is configured with proper credentials
4. ✅ Test user `testuser` is created and can authenticate
5. ✅ Application can verify JWT tokens from Keycloak
6. ✅ Protected endpoints require valid JWT tokens
7. ✅ Complete authentication flow works end-to-end

## 🎯 Next Steps

After completing Day 8:

1. **Enable Authentication**: Update application routes to use authentication middleware
2. **Test End-to-End**: Verify complete authentication flow
3. **Production Hardening**: Configure proper TLS and security settings
4. **Monitoring**: Add authentication metrics and logging
5. **Documentation**: Update API documentation with authentication requirements

## 📝 Notes

- Keycloak is configured for development mode (`start-dev`)
- Database credentials are hardcoded for simplicity
- In production, use proper secrets management
- Consider enabling HTTPS for production deployments
- Monitor Keycloak performance and resource usage

