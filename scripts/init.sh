#!/bin/bash

# Initialize the development environment

# Install dependencies
cd app
npm install

# Initialize Terraform
cd ../terraform
terraform init
