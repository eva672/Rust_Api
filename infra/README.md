# Infrastructure as Code

This directory contains the infrastructure code for the Rust API project, implementing Task 1: Summon the Cluster Beasts.

## Overview

The infrastructure setup creates a local K3s cluster with offline container registry capabilities using:

- **Terraform**: VM provisioning with Multipass
- **Ansible**: Configuration management and deployment
- **Local Registry**: TLS-secured container registry
- **Pre-pulled Images**: Offline-ready container images

## Quick Start

### Prerequisites

1. Install Multipass:

   ```bash
   # Ubuntu/Debian
   sudo snap install multipass

   # macOS
   brew install multipass
   ```

2. Install Terraform:

   ```bash
   # Ubuntu/Debian
   wget -O- https://apt.releases.hashicorp.com/gpg | gpg --dearmor | sudo tee /usr/share/keyrings/hashicorp-archive-keyring.gpg
   echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list
   sudo apt update && sudo apt install terraform
   ```

3. Install Ansible:

   ```bash
   sudo apt update && sudo apt install ansible
   ```

4. Generate SSH key pair:
   ```bash
   ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa
   ```

### Deploy Everything

```bash
# From the project root
cd infra/scripts
./deploy.sh
```

### Deploy Step by Step

1. **Generate certificates**:

   ```bash
   cd infra/scripts
   ./generate-certs.sh
   ```

2. **Deploy infrastructure**:

   ```bash
   cd infra/terraform
   terraform init
   terraform apply
   ```

3. **Configure cluster**:
   ```bash
   cd infra/ansible
   ansible-playbook -i inventory/hosts.yml playbooks/main.yml
   ```

## Directory Structure

```
infra/
├── terraform/              # VM provisioning
│   ├── main.tf            # Main Terraform configuration
│   ├── variables.tf       # Input variables
│   ├── outputs.tf         # Output values
│   ├── versions.tf        # Provider versions
│   ├── cloud-init.yml     # VM initialization
│   └── inventory.tpl      # Ansible inventory template
├── ansible/               # Configuration management
│   ├── inventory/         # Generated inventory files
│   ├── roles/            # Ansible roles
│   │   ├── common/       # Base system setup
│   │   ├── k3s/          # K3s installation
│   │   ├── registry/     # Local registry setup
│   │   ├── containerd/   # Registry mirrors
│   │   └── offline_prep/ # Image pre-pulling
│   ├── playbooks/        # Ansible playbooks
│   └── .artifacts/       # Generated files (kubeconfig, etc.)
├── scripts/              # Utility scripts
│   ├── deploy.sh         # Main deployment script
│   ├── generate-certs.sh # Certificate generation
│   └── image-list.txt    # Container images to pre-pull
└── README.md             # This file
```

## Configuration

### VM Configuration

Default VM settings (customizable in `terraform/variables.tf`):

- **Name**: k3s-cluster
- **CPU**: 2 cores
- **Memory**: 4GB
- **Disk**: 20GB
- **IP**: 192.168.64.100
- **OS**: Ubuntu 24.04 LTS

### Registry Configuration

- **Host**: registry.local
- **Port**: 5000
- **TLS**: Self-signed certificates
- **Storage**: `/opt/registry/data`

### Image Management

Images are managed through `scripts/image-list.txt`. The list includes:

- K3s core components
- Keycloak and PostgreSQL
- CloudNativePG operator
- Gitea
- Linkerd service mesh
- ArgoCD (for future GitOps)
- Rust application base images

## Usage

### Connect to the VM

```bash
multipass shell k3s-cluster
```

### Use kubectl

```bash
export KUBECONFIG=/home/ubuntu/.kube/config
kubectl get nodes
kubectl get pods -A
```

### Access the Registry

```bash
# List repositories
curl -k https://registry.local:5000/v2/_catalog

# Pull an image
docker pull registry.local:5000/nginx:1.25-alpine
```

### Trust the CA Certificate

On your host machine:

```bash
sudo cp certs/ca.crt /usr/local/share/ca-certificates/registry-ca.crt
sudo update-ca-certificates
```

## Troubleshooting

### VM Issues

```bash
# Check VM status
multipass list

# View VM logs
multipass logs k3s-cluster

# Restart VM
multipass restart k3s-cluster
```

### K3s Issues

```bash
# Check K3s service
sudo systemctl status k3s

# View K3s logs
sudo journalctl -u k3s -f

# Restart K3s
sudo systemctl restart k3s
```

### Registry Issues

```bash
# Check registry service
sudo systemctl status registry

# View registry logs
sudo journalctl -u registry -f

# Test registry connectivity
curl -k https://registry.local:5000/v2/_catalog
```

### Ansible Issues

```bash
# Test connectivity
ansible k3s_cluster -i inventory/hosts.yml -m ping

# Run specific role
ansible-playbook -i inventory/hosts.yml playbooks/main.yml --tags registry

# Verbose output
ansible-playbook -i inventory/hosts.yml playbooks/main.yml -vvv
```

## Cleanup

To destroy the entire infrastructure:

```bash
cd infra/terraform
terraform destroy
```

Or use the deployment script:

```bash
cd infra/scripts
./deploy.sh clean
```

## Security Notes

- Self-signed certificates are used for local development
- Registry runs with TLS encryption
- Proper file permissions are set on certificates
- CA certificate is added to system trust store

## Next Steps

After completing this infrastructure setup:

1. Deploy Keycloak for authentication
2. Set up CloudNativePG for PostgreSQL
3. Deploy the Rust API application
4. Configure service mesh with Linkerd
5. Implement GitOps with ArgoCD

## Support

For issues and questions:

1. Check the troubleshooting section above
2. Review the runbook documentation in `docs/runbooks/`
3. Check the architecture diagrams in `docs/diagrams/`







