### Day 1–2: Summon the Cluster Beasts – Runbook ✅ COMPLETED

This runbook documents how to provision a local K3s cluster on a Multipass VM, set up a local Docker registry, configure containerd mirrors, and pre-pull/push core images into the registry for offline use.

**Status**: ✅ **COMPLETED** - All infrastructure components are successfully deployed and tested.

#### Prerequisites (on your host)

- Multipass, Terraform (optional), Ansible, jq, curl
- Linux/macOS: ensure your user can run Ansible and SSH

Verify tools:

```bash
multipass version && ansible --version && jq --version && curl --version
```

#### 0) Repository layout (relevant parts)

```
app/
  Cargo.toml, Cargo.lock, Dockerfile, src/, migrations/
deploy/
  app/ (Rust app K8s manifests, config, secrets)
  keycloak/ (Keycloak K8s manifests)
  postgres/ (CNPG, DB cluster manifests)
infra/
  ansible/ (roles, playbooks, inventory)
  terraform/ (optional)
scripts/
  setup.sh, cleanup.sh, image-list.txt
```

### 1) Provision VM + K3s + Local Registry + Offline Images

End-to-end setup (idempotent):

```bash
cd /home/eva/Projects/rust_api
./scripts/setup.sh
```

What it does:

- Creates/ensures a Multipass VM `k3s-host-1`
- Installs base packages, disables swap, sets K3s sysctls
- Installs K3s (single-node) and kubectl
- Starts a local registry on the VM (`registry:2`, port 5000)
- Configures containerd mirrors for `localhost:5000` and `registry.local:5000`
- Mirrors images listed in `scripts/image-list.txt` into the local registry
- Adds `registry.local` to the VM `/etc/hosts` using the VM's primary IP (not 127.0.0.1)

Optional: enable Terraform path (default is skipped to avoid provider issues):

```bash
USE_TERRAFORM=1 ./scripts/setup.sh
```

### 2) Verify Cluster, Registry, and Images

SSH into the VM:

```bash
multipass shell k3s-host-1
```

K3s node status:

```bash
kubectl get nodes -o wide
```

Registry reachability (on the VM):

```bash
hostname -I | awk '{print $1}'            # shows <VM_IP>
getent hosts registry.local               # resolves to <VM_IP>
grep -E 'registry.local$' /etc/hosts      # line: <VM_IP> registry.local
curl -s http://registry.local:5000/v2/    # should return {}
```

List images mirrored into the registry catalog:

```bash
curl -s http://registry.local:5000/v2/_catalog | jq .
```

List tags for a given image (examples):

```bash
curl -s http://registry.local:5000/v2/keycloak/tags/list | jq .
curl -s http://registry.local:5000/v2/cloudnative-pg/tags/list | jq .
curl -s http://registry.local:5000/v2/proxy/tags/list | jq .
```

Confirm containerd mirror file exists (on VM):

```bash
sudo cat /etc/rancher/k3s/registries.yaml
```

### 3) Re-run subsets (tags)

You can run parts of the configuration idempotently by tags:

```bash
# Only base and system config
env -u SSH_AUTH_SOCK ANSIBLE_CONFIG=infra/ansible/ansible.cfg \
ansible-playbook -i infra/ansible/inventory/hosts.ini infra/ansible/playbooks/site.yml --tags common

# Only K3s setup
env -u SSH_AUTH_SOCK ANSIBLE_CONFIG=infra/ansible/ansible.cfg \
ansible-playbook -i infra/ansible/inventory/hosts.ini infra/ansible/playbooks/site.yml --tags k3s

# Only registry + mirrors
env -u SSH_AUTH_SOCK ANSIBLE_CONFIG=infra/ansible/ansible.cfg \
ansible-playbook -i infra/ansible/inventory/hosts.ini infra/ansible/playbooks/site.yml --tags registry,containerd

# Only offline image mirroring
env -u SSH_AUTH_SOCK ANSIBLE_CONFIG=infra/ansible/ansible.cfg \
ansible-playbook -i infra/ansible/inventory/hosts.ini infra/ansible/playbooks/site.yml --tags offline_prep
```

### 4) Update the Image List and Re-mirror

Edit `scripts/image-list.txt` with one image per line. Example contents (used for Task 1–2):

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

Re-run only the mirroring:

```bash
env -u SSH_AUTH_SOCK ANSIBLE_CONFIG=infra/ansible/ansible.cfg \
ansible-playbook -i infra/ansible/inventory/hosts.ini infra/ansible/playbooks/site.yml --tags offline_prep
```

### 5) Hostname Resolution on your laptop (optional)

If you want your host machine to resolve `registry.local`:

```bash
# Replace <VM_IP> with the IP of k3s-host-1 (see: multipass info k3s-host-1)
echo "<VM_IP> registry.local" | sudo tee -a /etc/hosts
```

### 6) Cleanup

Destroy the VM and clean up:

```bash
./scripts/cleanup.sh
```

### Troubleshooting Notes

- If pulling an image fails with unauthorized/manifest unknown, verify the tag exists upstream and no auth is needed.
- The registry is HTTP-only for local dev. Mirroring tasks push to `localhost:5000/...` inside the VM.
- If SSH issues occur, `./scripts/setup.sh` creates and uses a dedicated key at `infra/ansible/.keys/id_rsa_ansible` and injects it into the VM.
