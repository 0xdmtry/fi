# Use official Node.js 22 LTS image
FROM node:22.14.0

# Set working directory
WORKDIR /app

# Copy dependency files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy application source
COPY . .

# Start the Next.js app in development mode
CMD ["npm", "run", "dev"]
