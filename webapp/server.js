const express = require('express');
const http = require('http');
const { Server } = require('socket.io');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const readline = require('readline');

const app = express();
const server = http.createServer(app);
const io = new Server(server);

// Robust path calculation
const ENGINE_PATH = path.resolve(__dirname, '..', 'oxidized-fish', 'target', 'release', 'oxidized-fish.exe');

app.use(express.static('public'));

io.on('connection', (socket) => {
    console.log(`[${new Date().toLocaleTimeString()}] Client connected: ${socket.id}`);
    let engine = null;
    let rl = null;

    socket.on('start-engine', () => {
        if (engine) {
            console.log('Restarting engine for client...');
            engine.kill();
        }

        if (!fs.existsSync(ENGINE_PATH)) {
            console.error('Engine binary not found at:', ENGINE_PATH);
            socket.emit('engine-error', 'Chess engine binary not found. Please ensure it is built.');
            return;
        }

        try {
            engine = spawn(ENGINE_PATH);
            
            rl = readline.createInterface({
                input: engine.stdout,
                terminal: false
            });

            rl.on('line', (line) => {
                const trimmed = line.trim();
                if (trimmed) {
                    // console.log(`[Engine -> Client]: ${trimmed}`);
                    socket.emit('engine-response', trimmed);
                }
            });

            engine.stderr.on('data', (data) => {
                console.error(`[Engine Error]: ${data}`);
            });

            engine.on('error', (err) => {
                console.error('Failed to start engine process:', err);
                socket.emit('engine-error', 'Failed to start engine process');
            });

            engine.on('exit', (code) => {
                console.log(`Engine process exited with code ${code}`);
                socket.emit('engine-status', 'Engine stopped');
            });

            // Initial UCI handshake
            engine.stdin.write('uci\n');
            engine.stdin.write('isready\n');
            
        } catch (err) {
            console.error('Exception while spawning engine:', err);
            socket.emit('engine-error', 'Internal server error while starting engine');
        }
    });

    socket.on('engine-command', (command) => {
        if (engine && engine.stdin.writable) {
            // console.log(`[Client -> Engine]: ${command}`);
            engine.stdin.write(command + '\n');
        } else {
            console.warn('Attempted to send command to non-existent or non-writable engine');
            socket.emit('engine-error', 'Engine not running');
        }
    });

    socket.on('disconnect', () => {
        if (engine) {
            engine.kill();
            console.log(`[${new Date().toLocaleTimeString()}] Engine killed for disconnected client: ${socket.id}`);
        }
    });
});

const PORT = process.env.PORT || 3001;
server.listen(PORT, '0.0.0.0', () => {
    console.log(`
================================================
Chess Server is running!
URL: http://localhost:${PORT}
Engine Path: ${ENGINE_PATH}
================================================
`);
});