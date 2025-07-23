# Servermint Relay Server

This is a WebSocket relay server for Servermint that enables communication between the desktop app and remote VPS nodes.

## How It Works

The relay server acts as a central hub for all communication:

1. **Desktop App**: Connects to the relay server and authenticates with a user ID
2. **VPS Agents**: Connect to the relay server and authenticate with a token
3. **Message Routing**: The relay server forwards messages between desktop apps and their paired VPS nodes

This architecture eliminates the need for port forwarding or direct connections between the desktop app and VPS nodes.

## Features

- Secure WebSocket communication
- Token-based authentication for VPS nodes
- User-based ownership of nodes
- HTTP API for token generation
- Health check endpoint

## Requirements

- Node.js 16.x or later
- npm or yarn

## Installation

```bash
# Clone the repository (if applicable)
git clone https://github.com/yourusername/servermint-relay.git
cd servermint-relay

# Install dependencies
npm install
```

## Configuration

The server can be configured using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| PORT | Port to run the server on | 8080 |
| USE_HTTPS | Whether to use HTTPS | false |
| SSL_CERT | Path to SSL certificate | ./ssl/cert.pem |
| SSL_KEY | Path to SSL key | ./ssl/key.pem |
| LOG_LEVEL | Logging level (debug, info, warn, error) | info |

## Running Locally

```bash
# Start the server
npm start

# Start with development mode (auto-restart on changes)
npm run dev
```

## Deployment Options

### Option 1: Deploy to Heroku

```bash
# Install Heroku CLI
npm install -g heroku

# Login to Heroku
heroku login

# Create a new Heroku app
heroku create servermint-relay

# Push to Heroku
git push heroku main

# Check logs
heroku logs --tail
```

### Option 2: Deploy to DigitalOcean App Platform

1. Create a new app on DigitalOcean App Platform
2. Connect your GitHub repository
3. Set the environment variables
4. Deploy

### Option 3: Deploy to AWS EC2

1. Launch an EC2 instance
2. SSH into the instance
3. Clone the repository
4. Install Node.js and npm
5. Install dependencies with `npm install`
6. Use PM2 to manage the process:

```bash
npm install -g pm2
pm2 start server.js --name servermint-relay
pm2 startup
pm2 save
```

## Setting Up SSL/TLS

For production use, you should enable HTTPS:

1. Obtain SSL certificates (e.g., from Let's Encrypt)
2. Place the certificate and key files in the `./ssl` directory
3. Set the environment variables:

```bash
USE_HTTPS=true
SSL_CERT=/path/to/cert.pem
SSL_KEY=/path/to/key.pem
```

## API Endpoints

### GET /

Returns a simple welcome message.

### GET /health

Returns the health status of the server.

### POST /api/token

Generates a new pairing token for a VPS node.

**Request Body:**
```json
{
  "userId": "user-123"
}
```

**Response:**
```json
{
  "token": "sm-a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6"
}
```

## WebSocket Protocol

### Authentication

#### Desktop Client
```json
{
  "type": "Authenticate",
  "data": {
    "userId": "user-123"
  }
}
```

#### VPS Agent
```json
{
  "type": "Authenticate",
  "data": {
    "token": "sm-a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6"
  }
}
```

### Message Types

The relay server supports the following message types:

#### From Agent to Desktop
- `Authenticate`: Authentication with token
- `NodeInfo`: System information (CPU, memory, disk)
- `ServerList`: List of Minecraft servers
- `ServerStatus`: Status of a specific server
- `ServerLogs`: Logs from a specific server
- `CommandResult`: Result of a command execution
- `Error`: Error message

#### From Desktop to Agent
- `CreateServer`: Create a new Minecraft server
- `StartServer`: Start a server
- `StopServer`: Stop a server
- `SendCommand`: Send a command to a server
- `RequestLogs`: Request logs from a server
- `RequestServerList`: Request list of servers
- `RequestNodeInfo`: Request node information

## License

MIT 