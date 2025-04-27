export interface SwapRequest {
    inputMint: string;       // token to swap from (mint address)
    outputMint: string;      // token to swap to (mint address)
    amount: string;          // amount as string (human-readable, e.g., "0.5")
    slippage: number;        // slippage in % (e.g., 0.5 means 0.5%)
    userPublicKey: string;   // user public key (for token accounts)
}

export interface SwapResponse {
    transactionBase64: string; // built transaction ready to be signed (Base64 encoded)
}
