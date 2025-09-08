#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
TF_DIR="$ROOT_DIR/infra/terraform"
ANSIBLE_DIR="$ROOT_DIR/infra/ansible"
INV_PATH="$ANSIBLE_DIR/inventory/hosts.ini"
VM_NAME="k3s-host-1"
ANSIBLE_KEY_DIR="$ANSIBLE_DIR/.keys"
ANSIBLE_KEY="$ANSIBLE_KEY_DIR/id_rsa_ansible"

terraform_apply() {
  if [ "${USE_TERRAFORM:-0}" != "1" ]; then
    echo "Skipping Terraform (set USE_TERRAFORM=1 to enable)"
    return 1
  fi
  pushd "$TF_DIR" >/dev/null
  if terraform init -input=false; then
    if terraform apply -auto-approve -input=false; then
      terraform output -raw ansible_inventory > "$INV_PATH"
      popd >/dev/null
      return 0
    fi
  fi
  popd >/dev/null || true
  return 1
}

fallback_multipass_and_inventory() {
  if ! command -v multipass >/dev/null 2>&1; then
    echo "multipass is required for fallback mode" >&2
    exit 1
  fi
  if ! multipass list | grep -q "^$VM_NAME\s"; then
    echo "Creating Multipass VM: $VM_NAME"
    multipass launch 24.04 --name "$VM_NAME" --cpus 2 --memory 4G --disk 20G
  else
    echo "VM $VM_NAME already exists"
  fi
  # Ensure dedicated Ansible SSH key exists (no passphrase) and is installed on the VM
  mkdir -p "$ANSIBLE_KEY_DIR"
  if [ ! -f "$ANSIBLE_KEY" ]; then
    echo "Generating Ansible SSH key at $ANSIBLE_KEY"
    ssh-keygen -t rsa -b 4096 -N "" -f "$ANSIBLE_KEY" >/dev/null
  fi
  PUBKEY=$(cat "$ANSIBLE_KEY.pub")
  echo "Installing local SSH public key into VM authorized_keys"
  multipass exec "$VM_NAME" -- bash -lc "sudo install -d -m 700 -o ubuntu -g ubuntu ~ubuntu/.ssh && echo '$PUBKEY' | sudo tee -a ~ubuntu/.ssh/authorized_keys >/dev/null && sudo chown ubuntu:ubuntu ~ubuntu/.ssh/authorized_keys && sudo chmod 600 ~ubuntu/.ssh/authorized_keys"
  VM_IP=$(multipass info "$VM_NAME" --format json | jq -r ".info[\"$VM_NAME\"].ipv4[0]")
  mkdir -p "$(dirname "$INV_PATH")"
  cat > "$INV_PATH" <<EOF
[k3s]
$VM_NAME ansible_host=$VM_IP ansible_user=ubuntu ansible_ssh_common_args='-o StrictHostKeyChecking=no -o IdentitiesOnly=yes' ansible_ssh_private_key_file=$ANSIBLE_KEY

[all:vars]
ansible_python_interpreter=/usr/bin/python3
EOF
}

if ! terraform_apply; then
  echo "Terraform path failed or provider missing; using Multipass fallback" >&2
  fallback_multipass_and_inventory
fi

env -u SSH_AUTH_SOCK ANSIBLE_CONFIG="$ANSIBLE_DIR/ansible.cfg" ansible-playbook -i "$INV_PATH" "$ANSIBLE_DIR/playbooks/site.yml"

