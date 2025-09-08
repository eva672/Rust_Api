### Environment Variables

Create a `.env` file in `app/` with:

```
APP_HOST=0.0.0.0
APP_PORT=3000

# Postgres
DATABASE_URL=postgres://postgres:postgres@localhost:5432/rust_api

# Keycloak
KEYCLOAK_BASE_URL=http://keycloak.local
KEYCLOAK_REALM=rust-api-realm
KEYCLOAK_CLIENT_ID=rust-api-client
# Optional for introspection
KEYCLOAK_CLIENT_SECRET=
```

Notes:

- `DATABASE_URL` is used by SQLx to connect and run migrations at startup.
- `KEYCLOAK_*` drive JWT verification and token introspection.
