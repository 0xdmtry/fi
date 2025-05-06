import { Router } from 'express';
import { getOrcaSwapQuote } from '../services/orcaQuoteService';
import { QuoteRequest } from '../payloads/quote';

export const quoteRouter = Router();

quoteRouter.post('/', async (req, res) => {
    try {
        const quoteRequest: QuoteRequest = req.body;
        const quoteResponse = await getOrcaSwapQuote(quoteRequest);
        res.json(quoteResponse);
    } catch (error: any) {
        console.error(error);
        res.status(400).json({ error: error.message });
    }
});
