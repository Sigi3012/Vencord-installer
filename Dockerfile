# Use an official Node.js runtime as a base image
FROM node:latest

# Set the working directory in the container
WORKDIR /app

# Install git
RUN apt-get update && apt-get install -y git

# Install pnpm
RUN npm install -g pnpm

# Clone Vencord repository
RUN git clone https://github.com/Vendicated/Vencord

# Change directory to Vencord
WORKDIR /app/Vencord

COPY userplugins/ src/userplugins/

# Install dependencies
RUN pnpm install --frozen-lockfile

# Build Vencord
CMD [ "pnpm", "build" ]
