module "vpc" {
  source = "./modules/vpc"

  vpc_cidr        = var.vpc_cidr
  public_subnets  = var.public_subnets
  private_subnets = var.private_subnets
  app_port        = var.app_port
}

module "ecr" {
  source          = "./modules/ecr"
  repository_name = var.ecr_repository_name
}

module "rds" {
  source                    = "./modules/rds"
  vpc_id                    = module.vpc.vpc_id
  subnet_ids                = module.vpc.private_subnets_ids
  db_username               = var.db_username
  db_password               = var.db_password
  db_name                   = var.db_name
  security_group_ids        = [module.vpc.ecs_security_group_id]
  bastion_security_group_id = module.bastion.bastion_security_group_id
  bastion_public_ip         = module.bastion.bastion_public_ip
}

module "ecs" {
  source                = "./modules/ecs"
  cluster_name          = var.ecs_cluster_name
  vpc_id                = module.vpc.vpc_id
  subnet_ids            = module.vpc.private_subnets_ids
  security_group_ids    = [module.vpc.ecs_security_group_id]
  ecr_repository_url    = module.ecr.repository_url
  app_port              = var.app_port
  alb_security_group_id = module.vpc.alb_security_group_id
  desired_count         = var.desired_count
  db_endpoint           = module.rds.db_endpoint
  db_username           = var.db_username
  db_password           = var.db_password
  db_name               = var.db_name
  api_key               = var.api_key
  aws_region            = var.aws_region
  public_subnets        = module.vpc.public_subnets_ids
  uploads_bucket_arn    = module.s3.bucket_arn
}

module "s3" {
  source      = "./modules/s3"
  bucket_name = var.uploads_bucket_name
}

module "cognito" {
  source = "./modules/cognito"
}

module "bastion" {
  source           = "./modules/bastion"
  vpc_id           = module.vpc.vpc_id
  public_subnet_id = module.vpc.public_subnets_ids[0]
}
