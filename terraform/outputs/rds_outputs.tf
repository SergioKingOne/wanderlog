output "db_endpoint" {
  description = "The connection endpoint for the RDS instance"
  value       = module.rds.db_endpoint
}

output "db_security_group_id" {
  description = "Security Group ID for RDS"
  value       = module.rds.db_security_group_id
}
