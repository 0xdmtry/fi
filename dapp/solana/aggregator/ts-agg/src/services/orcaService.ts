import {Connection, PublicKey, Transaction} from '@solana/web3.js';
import {getOrca, OrcaPoolConfig, OrcaU64, Network} from '@orca-so/sdk';
import {DecimalUtil} from "@orca-so/common-sdk"; // ✅ Correct place
import {SwapRequest, SwapResponse} from '../payloads/swap';
import {POOL_MINTS_TO_CONFIG} from '../config/orcaPools';

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
const orca = getOrca(connection, Network.DEVNET);

export async function createSwapTransaction(swapRequest: SwapRequest): Promise<SwapResponse> {
    const {inputMint, outputMint, amount, slippage, userPublicKey} = swapRequest;

    const poolKey = `${inputMint}_${outputMint}`;
    const poolConfig = POOL_MINTS_TO_CONFIG[poolKey];

    if (!poolConfig) {
        throw new Error('Pool not supported for given mint pair');
    }

    const pool = orca.getPool(poolConfig);

    const inputToken = pool.getTokenA().mint.toBase58() === inputMint
        ? pool.getTokenA()
        : pool.getTokenB();

    const amountNumber = parseFloat(amount);

    if (isNaN(amountNumber)) {
        throw new Error('Invalid amount format');
    }

    const amountDecimal = DecimalUtil.fromNumber(amountNumber);

    const amountIn = OrcaU64.fromDecimal(amountDecimal, inputToken.scale);

    const slippageDecimal = DecimalUtil.fromNumber(slippage);

    const swapQuote = await pool.getQuote(inputToken, amountIn, slippageDecimal);

    const userPubkey = new PublicKey(userPublicKey);

    const swapPayload = await pool.swap(userPubkey, inputToken, amountIn, swapQuote.getMinOutputAmount());

    const transaction = swapPayload.transaction;

    const serializedTransaction = transaction.serialize({
        requireAllSignatures: false,
        verifySignatures: false,
    });

    const transactionBase64 = serializedTransaction.toString('base64');

    return {transactionBase64};
}
