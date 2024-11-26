output "cluster_id" {
  description = "The ID of the ECS Cluster"
  value       = aws_ecs_cluster.main.id
}

output "ecs_security_group_id" {
  description = "Security Group ID for ECS tasks"
  value       = aws_security_group.ecs_sg.id
}
