# Use lightweight Node.js image
FROM node:20-slim

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy source code
COPY . .

# Expose app port
EXPOSE 3000

# Run in development mode (live reload with nodemon)
CMD ["npm", "run", "dev"]
