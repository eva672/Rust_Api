output "inventory_path" {
  description = "Path where the rendered Ansible inventory should be written by wrapper scripts"
  value       = "${path.module}/../ansible/inventory/hosts.ini"
}
# removed duplicate outputs from legacy scaffold
