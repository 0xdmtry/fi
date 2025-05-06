export interface QuoteRequest {
    inputMint: string;
    outputMint: string;
    amount: string; // human-readable
    slippage: number; // percentage
}


export interface QuoteResponse {
    expectedOutputAmount: string;
    minimumOutputAmount: string;
    rate: string;
    pool: string;
}
