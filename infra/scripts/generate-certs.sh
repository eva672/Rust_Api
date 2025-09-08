#!/bin/bash

# Generate TLS certificates for local registry
# This script creates self-signed certificates for registry.local

set -e

CERT_DIR="../certs"
REGISTRY_DOMAIN="registry.local"

echo "Generating TLS certificates for local registry..."

# Create certs directory if it doesn't exist
mkdir -p "$CERT_DIR"

# Generate CA private key
echo "Generating CA private key..."
openssl genrsa -out "$CERT_DIR/ca.key" 4096

# Generate CA certificate
echo "Generating CA certificate..."
openssl req -new -x509 -days 365 -key "$CERT_DIR/ca.key" -out "$CERT_DIR/ca.crt" \
    -subj "/C=US/ST=Local/L=Local/O=Local Registry/OU=IT/CN=registry.local"

# Generate registry private key
echo "Generating registry private key..."
openssl genrsa -out "$CERT_DIR/registry.key" 4096

# Generate registry certificate signing request
echo "Generating registry CSR..."
openssl req -new -key "$CERT_DIR/registry.key" -out "$CERT_DIR/registry.csr" \
    -subj "/C=US/ST=Local/L=Local/O=Local Registry/OU=IT/CN=registry.local"

# Generate registry certificate
echo "Generating registry certificate..."
openssl x509 -req -days 365 -in "$CERT_DIR/registry.csr" -CA "$CERT_DIR/ca.crt" \
    -CAkey "$CERT_DIR/ca.key" -CAcreateserial -out "$CERT_DIR/registry.crt"

# Set proper permissions
chmod 600 "$CERT_DIR/ca.key" "$CERT_DIR/registry.key"
chmod 644 "$CERT_DIR/ca.crt" "$CERT_DIR/registry.crt"

# Clean up CSR file
rm "$CERT_DIR/registry.csr"

echo "Certificates generated successfully!"
echo "CA Certificate: $CERT_DIR/ca.crt"
echo "Registry Certificate: $CERT_DIR/registry.crt"
echo "Registry Private Key: $CERT_DIR/registry.key"
echo ""
echo "To trust the CA certificate on your host system:"
echo "sudo cp $CERT_DIR/ca.crt /usr/local/share/ca-certificates/registry-ca.crt"
echo "sudo update-ca-certificates"







