#!/bin/bash

# Deploy K3s cluster with local registry
# This script orchestrates the entire deployment process

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
TERRAFORM_DIR="$PROJECT_ROOT/infra/terraform"
ANSIBLE_DIR="$PROJECT_ROOT/infra/ansible"
CERTS_DIR="$PROJECT_ROOT/certs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if Multipass is installed
    if ! command -v multipass &> /dev/null; then
        log_error "Multipass is not installed. Please install it first."
        exit 1
    fi
    
    # Check if Terraform is installed
    if ! command -v terraform &> /dev/null; then
        log_error "Terraform is not installed. Please install it first."
        exit 1
    fi
    
    # Check if Ansible is installed
    if ! command -v ansible-playbook &> /dev/null; then
        log_error "Ansible is not installed. Please install it first."
        exit 1
    fi
    
    # Check if SSH key exists
    if [ ! -f ~/.ssh/id_rsa ]; then
        log_error "SSH private key not found at ~/.ssh/id_rsa"
        log_info "Generate one with: ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa"
        exit 1
    fi
    
    log_success "All prerequisites met!"
}

# Generate certificates
generate_certificates() {
    log_info "Generating TLS certificates..."
    
    if [ ! -d "$CERTS_DIR" ]; then
        mkdir -p "$CERTS_DIR"
    fi
    
    cd "$SCRIPT_DIR"
    ./generate-certs.sh
    
    log_success "Certificates generated!"
}

# Deploy infrastructure with Terraform
deploy_infrastructure() {
    log_info "Deploying infrastructure with Terraform..."
    
    cd "$TERRAFORM_DIR"
    
    # Initialize Terraform
    terraform init
    
    # Plan deployment
    terraform plan
    
    # Apply deployment
    terraform apply -auto-approve
    
    log_success "Infrastructure deployed!"
}

# Configure with Ansible
configure_with_ansible() {
    log_info "Configuring cluster with Ansible..."
    
    cd "$ANSIBLE_DIR"
    
    # Run Ansible playbook
    ansible-playbook -i inventory/hosts.yml playbooks/main.yml
    
    log_success "Cluster configured!"
}

# Verify deployment
verify_deployment() {
    log_info "Verifying deployment..."
    
    # Check if VM is running
    if multipass list | grep -q "k3s-cluster.*Running"; then
        log_success "VM is running"
    else
        log_error "VM is not running"
        return 1
    fi
    
    # Check K3s cluster
    log_info "Checking K3s cluster status..."
    multipass exec k3s-cluster -- bash -c "export KUBECONFIG=/home/ubuntu/.kube/config && kubectl get nodes"
    
    # Check registry
    log_info "Checking registry status..."
    multipass exec k3s-cluster -- curl -k -s https://registry.local:5000/v2/_catalog | jq .
    
    log_success "Deployment verified!"
}

# Display connection information
show_connection_info() {
    log_info "Deployment completed successfully!"
    echo ""
    echo "Connection Information:"
    echo "======================"
    echo "VM Name: k3s-cluster"
    echo "VM IP: 192.168.64.100"
    echo "Registry: https://registry.local:5000"
    echo ""
    echo "To connect to the VM:"
    echo "  multipass shell k3s-cluster"
    echo ""
    echo "To use kubectl:"
    echo "  export KUBECONFIG=/home/ubuntu/.kube/config"
    echo "  kubectl get nodes"
    echo ""
    echo "To access the registry:"
    echo "  curl -k https://registry.local:5000/v2/_catalog"
    echo ""
    echo "Kubeconfig location:"
    echo "  /home/ubuntu/.kube/config"
    echo ""
    echo "Certificate files:"
    echo "  CA: $CERTS_DIR/ca.crt"
    echo "  Registry: $CERTS_DIR/registry.crt"
}

# Main deployment function
main() {
    log_info "Starting K3s cluster deployment..."
    
    check_prerequisites
    generate_certificates
    deploy_infrastructure
    configure_with_ansible
    verify_deployment
    show_connection_info
    
    log_success "Deployment completed successfully!"
}

# Handle script arguments
case "${1:-}" in
    "certificates")
        generate_certificates
        ;;
    "infrastructure")
        deploy_infrastructure
        ;;
    "ansible")
        configure_with_ansible
        ;;
    "verify")
        verify_deployment
        ;;
    "clean")
        log_warning "Cleaning up deployment..."
        cd "$TERRAFORM_DIR"
        terraform destroy -auto-approve
        log_success "Cleanup completed!"
        ;;
    *)
        main
        ;;
esac
