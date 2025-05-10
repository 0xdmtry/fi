import axios from "axios";

export interface SignTransactionRequest {
    user_id: string; // UUID
    transaction_base64: string;
    wallet_id?: string; // optional
}

export interface SignTransactionResponse {
    signed_transaction_base64: string;
    signature: string;
    wallet_address?: string;
    wallet_public_key?: string;
}

const WALLETOR_BASE_URL = process.env.WALLETOR_BASE_URL || "http://host.docker.internal:8003"; // adjust port if needed


export async function requestSignatureFromWalletor(
    payload: SignTransactionRequest
): Promise<SignTransactionResponse> {
    try {
        const res = await axios.post<SignTransactionResponse>(
            `${WALLETOR_BASE_URL}/v1/sign`,
            payload,
            {
                headers: {
                    "Content-Type": "application/json",
                },
            }
        );
        return res.data;
    } catch (err: any) {
        console.error("❌ Failed to get signature from Walletor:", err?.response?.data || err.message);
        throw new Error("Walletor signing failed");
    }
}
