import {Router} from 'express';

export const pingRouter = Router();

pingRouter.get('/', (_req, res) => {
    res.json({message: 'ts-agg-v0.1.0'});
});
