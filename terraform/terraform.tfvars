aws_region          = "us-east-1"
vpc_cidr            = "10.0.0.0/16"
public_subnets      = ["10.0.1.0/24", "10.0.2.0/24"]
private_subnets     = ["10.0.11.0/24", "10.0.12.0/24"]
db_name             = "wanderlog"
ecs_cluster_name    = "wanderlog-cluster"
ecr_repository_name = "web-app-repo"
app_port            = 5000
desired_count       = 2

# TODO: These sensitive values should be set via environment variables or secure parameter store
# db_username         = "admin"
# db_password         = "SecurePassword123!"
# api_key             = "your-api-key"
# alb_security_group_id = "sg-xxx" 
