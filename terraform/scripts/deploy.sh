#!/bin/bash

set -e

# Initialize Terraform
terraform init

# Validate Terraform files
terraform validate

# Plan Terraform deployment
terraform plan -out=tfplan

# Apply Terraform deployment
terraform apply tfplan
