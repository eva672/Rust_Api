terraform {
  required_version = ">= 1.6.0"

  required_providers {
    multipass = {
      source  = "tristanpemble/multipass"
      version = ">= 1.5.0"
    }
    template = {
      source  = "hashicorp/template"
      version = ">= 2.2.0"
    }
  }
}

provider "multipass" {}

locals {
  vm_name   = var.vm_name
  ssh_user  = var.ssh_user
  ssh_key   = var.ssh_public_key
}

resource "multipass_instance" "k3s_host" {
  name           = local.vm_name
  cpus           = var.vm_cpus
  memory         = var.vm_memory
  disk           = var.vm_disk
  image          = var.vm_image
  cloudinit_file = var.cloud_init_file
}

data "template_file" "ansible_inventory" {
  template = file("${path.module}/templates/hosts.j2")

  vars = {
    host_name    = multipass_instance.k3s_host.name
    host_ip      = multipass_instance.k3s_host.ipv4[0]
    ssh_user     = local.ssh_user
    ssh_pub_key  = file(local.ssh_key)
  }
}

output "vm_name" {
  value = multipass_instance.k3s_host.name
}

output "vm_ip" {
  value = multipass_instance.k3s_host.ipv4[0]
}

output "ansible_inventory" {
  value = data.template_file.ansible_inventory.rendered
}
