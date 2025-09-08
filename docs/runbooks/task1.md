### Task 1: Summon the Cluster Beasts (VM + K3s + Registry + Offline Images)

Steps:

- Terraform provisions a Multipass VM.
- Ansible installs base packages, K3s, local registry, and configures containerd mirrors.
- Ansible pre-pulls images defined in `scripts/image-list.txt` and pushes them to the local registry.

Commands:

```bash
./scripts/setup.sh
```

Cleanup:

```bash
./scripts/cleanup.sh
```

Artifacts:

- Inventory rendered to `infra/ansible/inventory/hosts.ini`.
- Kubeconfig on VM at `/root/.kube/config`.

Notes:

- Registry listens on `localhost:5000` on the VM. Access within the VM or via port-forward/ssh tunnel if needed.

