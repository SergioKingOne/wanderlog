variable "vpc_id" {
  description = "VPC ID where RDS will be deployed"
  type        = string
}

variable "subnet_ids" {
  description = "List of subnet IDs for RDS"
  type        = list(string)
}

variable "db_username" {
  description = "Database admin username"
  type        = string
}

variable "db_password" {
  description = "Database admin password"
  type        = string
  sensitive   = true
}

variable "db_name" {
  description = "Initial database name"
  type        = string
}

variable "security_group_ids" {
  description = "List of security group IDs to allow access to RDS"
  type        = list(string)
}

variable "bastion_security_group_id" {
  description = "Security group ID of the bastion host"
  type        = string
}

variable "bastion_public_ip" {
  description = "Public IP of the bastion host"
  type        = string
}
