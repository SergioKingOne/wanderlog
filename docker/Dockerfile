# Use an official Node runtime as a parent image
FROM node:18-alpine

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY app/package*.json ./

# Install dependencies
RUN npm install --production

# Copy the rest of the application code
COPY app/src ./src

# Copy environment variables
COPY app/.env.example ./.env

# Expose port
EXPOSE 5000

# Start the application
CMD ["npm", "start"]