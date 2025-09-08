# Task 1: Summon the Cluster Beasts

## Overview

This runbook covers the setup of a local K3s cluster with offline container registry capabilities using Multipass VMs, Terraform, and Ansible.

## Prerequisites

- Multipass installed and running
- Terraform >= 1.0
- Ansible >= 2.9
- SSH key pair generated (`~/.ssh/id_rsa` and `~/.ssh/id_rsa.pub`)

## Architecture

The setup creates:

- 1 Multipass VM (2 CPU, 4GB RAM, 20GB disk)
- K3s single-node cluster
- Local Docker registry with TLS
- Pre-pulled container images for offline use
- Proper DNS resolution for `registry.local`

## Quick Start

### 1. Generate TLS Certificates

```bash
cd infra/scripts
./generate-certs.sh
```

### 2. Provision Infrastructure

```bash
cd infra/terraform
terraform init
terraform plan
terraform apply
```

### 3. Configure and Deploy

```bash
cd infra/ansible
ansible-playbook -i inventory/hosts.yml playbooks/main.yml
```

### 4. Verify Setup

```bash
# Connect to the VM
multipass shell k3s-cluster

# Check K3s status
export KUBECONFIG=/home/ubuntu/.kube/config
kubectl get nodes

# Check registry
curl -k https://registry.local:5000/v2/_catalog
```

## Detailed Steps

### Infrastructure Provisioning (Terraform)

The Terraform configuration:

- Creates a Multipass VM with specified resources
- Configures static IP (192.168.64.100)
- Sets up cloud-init for initial configuration
- Generates Ansible inventory automatically
- Mounts the project directory for development

### Configuration Management (Ansible)

Ansible roles execute in sequence:

1. **common**: Base system setup

   - Updates packages
   - Disables swap
   - Configures sysctls for Kubernetes
   - Sets up networking

2. **registry**: Local Docker registry

   - Generates TLS certificates
   - Creates systemd service
   - Configures registry with TLS

3. **containerd**: Registry mirrors

   - Configures K3s registry mirrors
   - Sets up certificate trust
   - Points to local registry

4. **k3s**: Kubernetes cluster

   - Installs K3s
   - Configures kubeconfig
   - Installs kubectl

5. **offline_prep**: Image mirroring
   - Pulls images from the list
   - Tags for local registry
   - Pushes to local registry

### Image Management

Images are managed through `infra/scripts/image-list.txt`:

- Core K3s components
- Keycloak and PostgreSQL
- CloudNativePG operator
- Gitea
- Linkerd service mesh
- ArgoCD (for future GitOps)
- Rust application base images

## Troubleshooting

### VM Not Starting

```bash
multipass list
multipass logs k3s-cluster
```

### K3s Not Ready

```bash
multipass shell k3s-cluster
sudo systemctl status k3s
sudo journalctl -u k3s -f
```

### Registry Not Accessible

```bash
# Check registry service
sudo systemctl status registry

# Check certificates
ls -la /home/ubuntu/rust_api/certs/

# Test connectivity
curl -k https://registry.local:5000/v2/_catalog
```

### Images Not Pulling

```bash
# Check Docker daemon
sudo systemctl status docker

# Check registry configuration
cat /etc/rancher/k3s/registries.yaml

# Test image pull manually
docker pull registry.local:5000/nginx:1.25-alpine
```

## Security Considerations

- Self-signed certificates for local development
- Registry runs with TLS encryption
- Proper file permissions on certificates
- CA certificate added to system trust store

## Next Steps

After completing this task:

1. Deploy Keycloak for authentication
2. Set up CloudNativePG for PostgreSQL
3. Deploy the Rust API application
4. Configure service mesh with Linkerd
5. Implement GitOps with ArgoCD

## File Structure

```
infra/
├── terraform/           # VM provisioning
├── ansible/            # Configuration management
├── scripts/            # Utility scripts
└── certs/              # TLS certificates

docs/
├── runbooks/           # This documentation
└── diagrams/           # Architecture diagrams
```

## Variables

Key variables can be customized in:

- `infra/terraform/variables.tf` - VM configuration
- `infra/terraform/inventory.tpl` - Ansible variables
- `infra/scripts/image-list.txt` - Container images







