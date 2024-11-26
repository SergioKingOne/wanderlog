resource "aws_db_instance" "default" {
  identifier                 = "my-web-app-db"
  engine                     = "postgres"
  engine_version             = "14.5"
  instance_class             = "db.t3.micro"
  allocated_storage          = 20
  storage_type               = "gp2"
  name                       = var.db_name
  username                   = var.db_username
  password                   = var.db_password
  parameter_group_name       = "default.postgres14"
  skip_final_snapshot        = true
  publicly_accessible        = false
  vpc_security_group_ids     = [aws_security_group.rds_sg.id]
  db_subnet_group_name       = aws_db_subnet_group.main.name
  multi_az                   = false
  auto_minor_version_upgrade = true
  backup_retention_period    = 7

  tags = {
    Name = "my-web-app-db"
  }
}

resource "aws_db_subnet_group" "main" {
  name       = "my-web-app-db-subnet-group"
  subnet_ids = var.subnet_ids

  tags = {
    Name = "my-web-app-db-subnet-group"
  }
}

resource "aws_security_group" "rds_sg" {
  name        = "rds-security-group"
  description = "Allow inbound traffic for RDS"
  vpc_id      = var.vpc_id

  ingress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [var.ecs_security_group_id]
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
