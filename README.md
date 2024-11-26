# Cloudfusion

## Overview

**Cloudfusion** is a scalable, containerized web application deployed on AWS using Amazon RDS for relational database management and Terraform for Infrastructure as Code (IaC). The application is built with Node.js and Express.js, ensuring a robust and efficient backend.

## Features

- **CRUD Operations**: Manage users with Create, Read, Update, and Delete functionalities.
- **User Authentication**: Secure API endpoints using API keys.
- **Containerization**: Dockerized application for consistency across environments.
- **Infrastructure as Code**: Terraform scripts to provision AWS resources.
- **CI/CD Pipeline**: Automated build, test, and deployment workflows using GitHub Actions.
- **Monitoring & Logging**: Integrated with AWS CloudWatch for monitoring application performance.

## Technology Stack

- **Backend**: Node.js, Express.js
- **Database**: PostgreSQL on Amazon RDS
- **Containerization**: Docker
- **Orchestration**: AWS ECS with Fargate
- **Infrastructure as Code**: Terraform
- **CI/CD**: GitHub Actions
- **Monitoring**: AWS CloudWatch

## Getting Started

### Prerequisites

- AWS Account
- AWS CLI installed and configured
- Terraform installed
- Docker installed
- Node.js and npm installed

### Setup Instructions

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/cloudfusion.git
   cd cloudfusion
   ```

2. **Configure Environment Variables**

   Copy `.env.example` to `.env` and fill in the required variables.

   ```bash
   cp app/.env.example app/.env
   ```

3. **Initialize Terraform**

   ```bash
   cd terraform
   terraform init
   ```

4. **Deploy Infrastructure**

   ```bash
   terraform apply
   ```

5. **Build and Push Docker Image**

   ```bash
   cd ..
   ./scripts/build.sh
   ```

6. **Deploy Application**

   ```bash
   ./scripts/deploy.sh
   ```

7. **Access the Application**

   Navigate to the ALB's DNS name provided in the Terraform outputs.

## Testing

Run unit, integration, and end-to-end tests using Jest.

```bash
cd app
npm test
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.
