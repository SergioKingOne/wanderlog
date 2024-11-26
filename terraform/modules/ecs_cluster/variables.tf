variable "cluster_name" {
  description = "Name of the ECS Cluster"
  type        = string
}

variable "vpc_id" {
  description = "VPC ID where the cluster will be deployed"
  type        = string
}

variable "subnet_ids" {
  description = "List of subnet IDs for the ECS cluster"
  type        = list(string)
}

variable "app_port" {
  description = "Port on which the application runs"
  type        = number
}
