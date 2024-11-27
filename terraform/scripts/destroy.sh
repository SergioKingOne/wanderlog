#!/bin/bash

set -e

read -p "Are you sure you want to destroy the infrastructure? This action cannot be undone. (y/N): " confirm
if [[ $confirm != [yY] ]]; then
    echo "Destruction cancelled."
    exit 1
fi

echo "Initializing Terraform..."
terraform init

echo "Destroying Terraform-managed infrastructure..."
terraform destroy -auto-approve

echo "Infrastructure destroyed successfully."