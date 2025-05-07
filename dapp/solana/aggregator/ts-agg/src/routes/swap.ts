import {Router} from "express";
import {buildUnsignedSwapTransaction} from "../services/orcaService";
import {SwapRequest} from "../payloads/swap";

export const swapRouter = Router();

swapRouter.post("/", async (req, res) => {
    try {
        const request: SwapRequest = req.body;
        const result = await buildUnsignedSwapTransaction(request);
        res.json(result);
    } catch (err: any) {
        console.error(err);
        res.status(400).json({error: err.message});
    }
});
