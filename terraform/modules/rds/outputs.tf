output "db_endpoint" {
  description = "The connection endpoint for the RDS instance"
  value       = aws_db_instance.default.endpoint
}

output "security_group_id" {
  description = "The ID of the RDS security group"
  value       = aws_security_group.rds_sg.id
}
