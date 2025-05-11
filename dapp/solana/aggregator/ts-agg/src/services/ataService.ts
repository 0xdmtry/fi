import axios from "axios";

export interface CreateAtaRequest {
    user_id: string; // UUID
    token_mint: string;
    wallet_id?: string; // optional
}

export interface CreateAtaResponse {
    ata_address: string;
    tx_signature: string;
}

const WALLETOR_BASE_URL = process.env.WALLETOR_BASE_URL || "http://host.docker.internal:8003";

export async function requestAtaCreationFromWalletor(
    payload: CreateAtaRequest
): Promise<CreateAtaResponse> {


    try {
        const res = await axios.post<CreateAtaResponse>(
            `${WALLETOR_BASE_URL}/v1/create-ata`,
            payload,
            {
                headers: {
                    "Content-Type": "application/json",
                },
            }
        );


        return res.data;
    } catch (err: any) {
        console.error("❌ Failed to create ATA via Walletor:", err?.response?.data || err.message);
        throw new Error("Walletor ATA creation failed");
    }
}
