import {Router} from 'express';
import {createSwapTransaction} from '../services/orcaService';
import {SwapRequest} from '../payloads/swap';

export const swapRouter = Router();

swapRouter.post('/', async (req, res) => {
    try {
        const swapRequest: SwapRequest = req.body;
        const swapResponse = await createSwapTransaction(swapRequest);
        res.json(swapResponse);
    } catch (error: any) {
        console.error(error);
        res.status(400).json({error: error.message});
    }
});
