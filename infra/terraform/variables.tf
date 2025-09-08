variable "vm_name" {
  description = "Name of the Multipass VM"
  type        = string
  default     = "k3s-host-1"
}

variable "vm_cpus" {
  description = "Number of CPUs for the VM"
  type        = number
  default     = 2
}

variable "vm_memory" {
  description = "Memory allocation for VM (e.g., 8G)"
  type        = string
  default     = "4G"
}

variable "vm_disk" {
  description = "Disk size for VM (e.g., 50G)"
  type        = string
  default     = "20G"
}

variable "vm_image" {
  description = "Base image for Multipass VM"
  type        = string
  default     = "ubuntu-24.04"
}

variable "ssh_user" {
  description = "SSH username to connect with"
  type        = string
  default     = "ubuntu"
}

variable "ssh_public_key" {
  description = "Path to SSH public key to inject into inventory"
  type        = string
  default     = "~/.ssh/id_rsa.pub"
}

variable "cloud_init_file" {
  description = "Optional cloud-init file for initial provisioning"
  type        = string
  default     = null
}
# remove duplicate legacy variables
