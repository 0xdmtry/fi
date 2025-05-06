import {createSolanaRpc, devnet, address} from "@solana/kit";
import {
    setWhirlpoolsConfig,
    swapInstructions,
    type SwapInstructions,
    type SwapParams,
} from "@orca-so/whirlpools";
import Decimal from "decimal.js";
import {QuoteRequest, QuoteResponse} from "../payloads/quote";

// 1. Setup RPC and SDK config
const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));

export async function initWhirlpoolSdk() {
    await setWhirlpoolsConfig("solanaDevnet");
}

// 2. Known pool: SOL -> devUSDC
const POOL_ADDRESS = address("3KBZiL2g8C7tiJ32hTv5v3KM7aK9htpqTw4cTXz1HvPt");
const WSOL_MINT = address("So11111111111111111111111111111111111111112");
const WSOL_DECIMALS = 9;
const USDC_DECIMALS = 6;

export async function getOrcaSwapQuote(
    req: QuoteRequest
): Promise<QuoteResponse> {
    const {inputMint, amount, slippage} = req;

    if (inputMint !== WSOL_MINT.toString()) {
        throw new Error("Only WSOL input is supported in this example.");
    }

    const inputAmountDecimal = new Decimal(amount);
    const inputAmountRaw = BigInt(
        inputAmountDecimal.mul(Decimal.pow(10, WSOL_DECIMALS)).toFixed(0)
    );

    const slippageBps = Math.floor(slippage * 100);

    const params: SwapParams = {
        inputAmount: inputAmountRaw,
        mint: WSOL_MINT,
    };

    const {quote}: SwapInstructions<typeof params> = await swapInstructions(
        rpc,
        params,
        POOL_ADDRESS,
        slippageBps
    );

    const expectedOut = new Decimal(quote.tokenEstOut.toString()).div(
        Decimal.pow(10, USDC_DECIMALS)
    );

    const minOut = new Decimal(quote.tokenMinOut.toString()).div(
        Decimal.pow(10, USDC_DECIMALS)
    );

    const rate = expectedOut.div(inputAmountDecimal);

    return {
        expectedOutputAmount: expectedOut.toFixed(6),
        minimumOutputAmount: minOut.toFixed(6),
        rate: rate.toFixed(6),
        pool: POOL_ADDRESS.toString(),
    };
}
