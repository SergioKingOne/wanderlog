# terraform/modules/vpc/outputs.tf

output "vpc_id" {
  description = "The ID of the VPC"
  value       = aws_vpc.main.id
}

output "public_subnets_ids" {
  description = "List of public subnet IDs"
  value       = aws_subnet.public[*].id
}

output "private_subnets_ids" {
  description = "List of private subnet IDs"
  value       = aws_subnet.private[*].id
}

output "ecs_security_group_id" {
  description = "Security Group ID for ECS tasks"
  value       = aws_security_group.ecs_sg.id
}

output "rds_security_group_id" {
  description = "Security Group ID for RDS"
  value       = aws_security_group.rds_sg.id
}
