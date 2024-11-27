provider "aws" {
  region = var.aws_region
}

terraform {
  required_version = ">= 1.0.0"

  backend "local" {
    path = "terraform.tfstate"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}
