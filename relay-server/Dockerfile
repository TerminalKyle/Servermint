FROM node:18-alpine

# Create app directory
WORKDIR /app

# Install app dependencies
COPY package*.json ./
RUN npm ci --only=production

# Bundle app source
COPY . .

# Create SSL directory
RUN mkdir -p ssl

# Expose port
EXPOSE 8080

# Start the server
CMD ["node", "server.js"] 