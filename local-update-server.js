const http = require('http');
const fs = require('fs');
const path = require('path');

const server = http.createServer((req, res) => {
  // Set CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.url === '/updates.json') {
    // Serve the updates.json file
    const updatesPath = path.join(__dirname, 'updates.json');
    
    if (fs.existsSync(updatesPath)) {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(fs.readFileSync(updatesPath));
    } else {
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('updates.json not found');
    }
  } else {
    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end('Not found');
  }
});

const PORT = 3000;
server.listen(PORT, () => {
  console.log(`Local update server running at http://localhost:${PORT}`);
  console.log(`Updates available at http://localhost:${PORT}/updates.json`);
}); 