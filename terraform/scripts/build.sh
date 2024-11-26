#!/bin/bash

# Build the Docker image
docker build -t my-web-app:latest -f docker/Dockerfile .

# Tag the Docker image for ECR
aws_account_id=$(aws sts get-caller-identity --query Account --output text)
region=$(terraform output -raw aws_region)
ecr_repo=$(terraform output -raw ecr_repository_url)
docker tag my-web-app:latest ${aws_account_id}.dkr.ecr.${region}.amazonaws.com/${ecr_repo}:latest

# Push the Docker image to ECR
aws ecr get-login-password --region ${region} | docker login --username AWS --password-stdin ${aws_account_id}.dkr.ecr.${region}.amazonaws.com
docker push ${aws_account_id}.dkr.ecr.${region}.amazonaws.com/${ecr_repo}:latest