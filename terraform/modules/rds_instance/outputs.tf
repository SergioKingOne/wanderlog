output "db_endpoint" {
  description = "The connection endpoint for the RDS instance"
  value       = aws_db_instance.default.endpoint
}

output "db_security_group_id" {
  description = "Security Group ID for RDS"
  value       = aws_security_group.rds_sg.id
}
