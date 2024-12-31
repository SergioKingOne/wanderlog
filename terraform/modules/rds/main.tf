resource "aws_db_subnet_group" "main" {
  name       = "wanderlog-db-subnet-group"
  subnet_ids = var.subnet_ids

  tags = {
    Name = "wanderlog-db-subnet-group"
  }
}

resource "aws_security_group" "rds_sg" {
  name        = "rds-security-group"
  description = "Allow PostgreSQL traffic from ECS tasks"
  vpc_id      = var.vpc_id

  ingress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = concat(var.security_group_ids, [var.bastion_security_group_id])
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "rds-sg"
  }
}

resource "aws_db_instance" "default" {
  identifier                 = "wanderlog"
  engine                     = "postgres"
  engine_version             = "16.3"
  instance_class             = "db.t3.micro"
  allocated_storage          = 20
  storage_type               = "gp2"
  username                   = var.db_username
  password                   = var.db_password
  parameter_group_name       = "default.postgres16"
  skip_final_snapshot        = true
  publicly_accessible        = false
  vpc_security_group_ids     = [aws_security_group.rds_sg.id]
  db_subnet_group_name       = aws_db_subnet_group.main.name
  multi_az                   = false
  auto_minor_version_upgrade = true
  backup_retention_period    = 7

  tags = {
    Name = "wanderlog-db"
  }
}

# Add null_resource for database initialization
resource "null_resource" "db_init" {
  depends_on = [aws_db_instance.default]

  triggers = {
    db_instance = aws_db_instance.default.id
  }

  provisioner "local-exec" {
    command = <<-EOF
      # Wait for DB to be available
      sleep 30
      
      # Get bastion private key from SSM
      aws ssm get-parameter --name "/bastion/ssh_private_key" --with-decryption --query "Parameter.Value" --output text > bastion.pem
      chmod 600 bastion.pem
      
      # Copy schema file to bastion
      scp -i bastion.pem -o StrictHostKeyChecking=no ../app/src/db/schema.sql ec2-user@${var.bastion_public_ip}:/tmp/schema.sql
      
      # Execute schema
      ssh -i bastion.pem -o StrictHostKeyChecking=no ec2-user@${var.bastion_public_ip} \
        "PGPASSWORD='${var.db_password}' psql \
        -h ${aws_db_instance.default.endpoint} \
        -U ${var.db_username} \
        -d ${var.db_name} \
        -f /tmp/schema.sql"
      
      # Cleanup
      rm bastion.pem
    EOF
  }
}
