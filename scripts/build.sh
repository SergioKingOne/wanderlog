#!/bin/bash

# Load environment variables
if [ -f .env ]; then
    source .env
else
    echo "Error: .env file not found"
    exit 1
fi

# Export AWS credentials for AWS CLI
export AWS_ACCESS_KEY_ID
export AWS_SECRET_ACCESS_KEY
export AWS_REGION

# Validate required variables
required_vars=("AWS_REGION" "ECR_REPOSITORY_NAME" "APP_NAME" "AWS_ACCESS_KEY_ID" "AWS_SECRET_ACCESS_KEY")
for var in "${required_vars[@]}"; do
    if [ -z "${!var}" ]; then
        echo "Error: $var is not set in .env file"
        exit 1
    fi
done

# Build the Docker image
docker build -t ${APP_NAME}:latest -f docker/Dockerfile .

# Get AWS account ID
aws_account_id=$(aws sts get-caller-identity --query Account --output text)
if [ $? -ne 0 ]; then
    echo "Error: Failed to get AWS account ID. AWS credentials may be invalid."
    exit 1
fi

# Construct ECR repository URL
ecr_url="${aws_account_id}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPOSITORY_NAME}"

# Tag the Docker image for ECR
docker tag ${APP_NAME}:latest ${ecr_url}:latest

# Push the Docker image to ECR
echo "Logging into ECR..."
aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin ${aws_account_id}.dkr.ecr.${AWS_REGION}.amazonaws.com

if [ $? -eq 0 ]; then
    echo "Pushing image to ECR..."
    docker push ${ecr_url}:latest
else
    echo "Error: Failed to login to ECR"
    exit 1
fi