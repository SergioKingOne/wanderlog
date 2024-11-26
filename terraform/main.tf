module "vpc" {
  source          = "./modules/vpc"
  vpc_cidr        = var.vpc_cidr
  public_subnets  = var.public_subnets
  private_subnets = var.private_subnets
}

module "ecr" {
  source          = "./ecr"
  repository_name = var.ecr_repository_name
}

module "rds" {
  source      = "./rds"
  vpc_id      = module.vpc.vpc_id
  subnet_ids  = module.vpc.private_subnets_ids
  db_username = var.db_username
  db_password = var.db_password
  db_name     = var.db_name
}

module "ecs_cluster" {
  source       = "./ecs_cluster"
  cluster_name = var.ecs_cluster_name
  vpc_id       = module.vpc.vpc_id
  subnet_ids   = module.vpc.private_subnets_ids
}

module "alb" {
  source             = "./ecs/alb"
  vpc_id             = module.vpc.vpc_id
  public_subnets     = module.vpc.public_subnets_ids
  ecs_cluster_name   = module.ecs_cluster.cluster_name
  app_port           = var.app_port
  desired_count      = var.desired_count
  ecr_repository_url = module.ecr.repository_url
  db_endpoint        = module.rds.db_endpoint
}
