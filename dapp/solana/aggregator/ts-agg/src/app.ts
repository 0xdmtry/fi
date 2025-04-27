import express from 'express';
import {pingRouter} from './routes/ping';

const app = express();

// Middlewares (if any) can go here

// Routes
app.use('/v1/ping', pingRouter);

export {app};
