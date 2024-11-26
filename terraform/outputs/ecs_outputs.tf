output "ecs_cluster_id" {
  description = "The ID of the ECS Cluster"
  value       = module.ecs_cluster.cluster_id
}

output "ecs_security_group_id" {
  description = "Security Group ID for ECS tasks"
  value       = module.ecs_cluster.ecs_security_group_id
}
