#!/bin/bash

# Deploy the application to AWS

# Navigate to Terraform directory
cd terraform

# Deploy infrastructure
./scripts/deploy.sh

# Build and push Docker image
../scripts/build.sh

# Update ECS service
aws ecs update-service \
    --cluster $(terraform output -raw ecs_cluster_id) \
    --service my-web-app-service \
    --force-new-deployment
