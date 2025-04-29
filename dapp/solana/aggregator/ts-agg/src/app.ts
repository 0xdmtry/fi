import express from 'express';
import {pingRouter} from './routes/ping';
import {swapRouter} from './routes/swap';
import client from 'prom-client'; // <-- import prom-client

const app = express();

// Setup body parsing middleware
app.use(express.json());

// Setup Prometheus metrics collection
const collectDefaultMetrics = client.collectDefaultMetrics;
collectDefaultMetrics();

// /metrics endpoint
app.get('/metrics', async (_req, res) => {
    res.set('Content-Type', client.register.contentType);
    res.end(await client.register.metrics());
});

const httpRequestCounter = new client.Counter({
    name: 'http_requests_total',
    help: 'Total number of HTTP requests',
    labelNames: ['method', 'path', 'status'],
});

// Middleware to count requests
app.use((req, res, next) => {
    res.on('finish', () => {
        httpRequestCounter.inc({
            method: req.method,
            path: req.route?.path || req.path || 'unknown_path',
            status: res.statusCode,
        });
    });
    next();
});

// Routes
app.use('/v1/ping', pingRouter);
app.use('/v1/swap', swapRouter);

export {app};
