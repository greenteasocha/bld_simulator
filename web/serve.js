#!/usr/bin/env node
/**
 * Simple HTTP Server for Cross Solver WebUI
 * 
 * Usage:
 *   node serve.js [port]
 *   
 * Default port: 8080
 */

import { createServer } from 'http';
import { readFileSync, existsSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join, extname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const PORT = process.argv[2] || 8080;

// MIME types
const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.wasm': 'application/wasm',
  '.json': 'application/json',
  '.css': 'text/css',
};

const server = createServer((req, res) => {
  // Remove query string and decode URI
  let filePath = req.url.split('?')[0];
  filePath = decodeURIComponent(filePath);

  // Default to index.html
  if (filePath === '/') {
    filePath = '/index.html';
  }

  // Resolve file path
  // Check if path starts with /pkg/ to access parent directory's pkg folder
  let fullPath;
  if (filePath.startsWith('/pkg/')) {
    // Access pkg directory in parent folder
    fullPath = join(__dirname, '..', filePath);
  } else {
    // Access files in web directory
    fullPath = join(__dirname, filePath);
  }

  // Security check: prevent directory traversal beyond project root
  const projectRoot = join(__dirname, '..');
  if (!fullPath.startsWith(projectRoot)) {
    res.writeHead(403);
    res.end('403 Forbidden');
    return;
  }

  // Check if file exists
  if (!existsSync(fullPath)) {
    res.writeHead(404);
    res.end('404 Not Found');
    console.log(`❌ Not Found: ${filePath}`);
    return;
  }

  try {
    // Read file
    const content = readFileSync(fullPath);
    
    // Get MIME type
    const ext = extname(fullPath);
    const mimeType = MIME_TYPES[ext] || 'application/octet-stream';

    // Set CORS headers (needed for WASM)
    res.writeHead(200, {
      'Content-Type': mimeType,
      'Access-Control-Allow-Origin': '*',
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    });

    res.end(content);
    console.log(`✅ Served: ${filePath} (${mimeType})`);
  } catch (error) {
    res.writeHead(500);
    res.end('500 Internal Server Error');
    console.error(`❌ Error serving ${filePath}:`, error.message);
  }
});

server.listen(PORT, () => {
  console.log('🎲 Cross Solver WebUI Server');
  console.log('================================');
  console.log(`🌐 Server running at: http://localhost:${PORT}`);
  console.log(`📁 Serving from: ${__dirname}`);
  console.log('\n💡 Press Ctrl+C to stop');
  console.log('================================\n');
});

server.on('error', (error) => {
  if (error.code === 'EADDRINUSE') {
    console.error(`❌ Port ${PORT} is already in use`);
    console.log(`💡 Try a different port: node serve.js 3000`);
  } else {
    console.error('❌ Server error:', error.message);
  }
  process.exit(1);
});
