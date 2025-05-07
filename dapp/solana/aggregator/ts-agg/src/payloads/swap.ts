export interface SwapRequest {
    inputMint: string;
    outputMint: string;
    amount: string;
    slippage: number;
    userPublicKey: string;
}

export interface SwapUnsignedResponse {
    transactionBase64: string;
    pool: string;
}
