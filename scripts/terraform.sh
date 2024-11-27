#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    set -o allexport
    source .env
    set +o allexport
else
    echo "Warning: .env file not found. Proceeding with existing environment variables."
fi

# Set working directory to terraform
cd terraform

# Run terraform commands with the passed arguments
terraform "$@"