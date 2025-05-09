import {
    setWhirlpoolsConfig,
    swapInstructions,
} from "@orca-so/whirlpools";

import {
    createSolanaRpc,
    devnet,
    address,
} from "@solana/kit";

import {
    Transaction,
    PublicKey,
    sendAndConfirmRawTransaction,
    TransactionInstruction,
    VersionedTransaction,
    TransactionMessage,
} from "@solana/web3.js";

import {
    AccountMeta,
} from "@solana/web3.js";

import Decimal from "decimal.js";
import {SwapRequest, SwapUnsignedResponse} from "../payloads/swap";

import {AccountRole, Address} from "@solana/kit";

// Setup the RPC and SDK config once
const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));

export async function initWhirlpoolSdk() {
    await setWhirlpoolsConfig("solanaDevnet");
}


// Orca Devnet Pool
const POOL_ADDRESS = address("3KBZiL2g8C7tiJ32hTv5v3KM7aK9htpqTw4cTXz1HvPt");
const WSOL_MINT = address("So11111111111111111111111111111111111111112");
const WSOL_DECIMALS = 9;

function toAccountMeta(account: { address: Address; role: AccountRole }): AccountMeta {
    return {
        pubkey: new PublicKey(account.address),
        isSigner: account.role === AccountRole.READONLY_SIGNER || account.role === AccountRole.WRITABLE_SIGNER,
        isWritable: account.role === AccountRole.WRITABLE || account.role === AccountRole.WRITABLE_SIGNER,
    };
}

// Define a type guard
function isIAccountMeta(
    acc: unknown
): acc is { pubkey: string; isSigner: boolean; isWritable: boolean } {
    return (
        typeof acc === "object" &&
        acc !== null &&
        "pubkey" in acc &&
        "isSigner" in acc &&
        "isWritable" in acc
    );
}

export async function buildUnsignedSwapTransaction(
    req: SwapRequest
): Promise<SwapUnsignedResponse> {

    console.log("buildUnsignedSwapTransaction");
    console.log("req", req);

    const {inputMint, amount, slippage, userPublicKey} = req;

    console.log("inputMint", inputMint);
    console.log("amount", amount);
    console.log("slippage", slippage);
    console.log("userPublicKey", userPublicKey);

    if (inputMint !== WSOL_MINT.toString()) {
        throw new Error("Only WSOL input is supported in this example.");
    }

    const inputAmountDecimal = new Decimal(amount);
    const inputAmountRaw = BigInt(
        inputAmountDecimal.mul(Decimal.pow(10, WSOL_DECIMALS)).toFixed(0)
    );
    const slippageBps = Math.floor(slippage * 100);

    console.log("inputAmountDecimal", inputAmountDecimal);
    console.log("inputAmountRaw", inputAmountRaw);
    console.log("slippageBps", slippageBps);

    // Call Orca to generate quote + instructions
    const {instructions} = await swapInstructions(
        rpc,
        {
            inputAmount: inputAmountRaw,
            mint: WSOL_MINT,
        },
        POOL_ADDRESS,
        slippageBps,
        {
            address: address(userPublicKey),
            // these next two functions will never be called since we're not signing here
            signAndSendTransactions: () => {
                throw new Error("signAndSendTransactions is not supported in MPC flow");
            },
        }
    );

    console.log("instructions", instructions);

    // Get latest blockhash
    const blockhashResponse = await rpc.getLatestBlockhash().send();
    const blockhash = blockhashResponse.value.blockhash;

    // Convert Orca instructions (IInstruction) to real TransactionInstructions
    const realInstructions: TransactionInstruction[] = instructions.map((ix) => {
        if (!ix.accounts || !ix.data) {
            throw new Error("Invalid instruction: missing accounts or data");
        }

        const keys: AccountMeta[] = ix.accounts.map(toAccountMeta);

        return new TransactionInstruction({
            programId: new PublicKey(ix.programAddress),
            keys,
            data: Buffer.from(ix.data),
        });
    });

    console.log("blockhashResponse", blockhashResponse);
    console.log("blockhash", blockhash);
    console.log("realInstructions", realInstructions);

    // Build v0 transaction
    const message = new TransactionMessage({
        payerKey: new PublicKey(userPublicKey),
        recentBlockhash: blockhash,
        instructions: realInstructions,
    }).compileToV0Message();

    const unsignedTx = new VersionedTransaction(message);

    const txBase64 = Buffer.from(unsignedTx.serialize()).toString("base64");

    console.log("message", message);
    console.log("unsignedTx", unsignedTx);
    console.log("txBase64", txBase64);

    return {
        transactionBase64: txBase64,
        pool: POOL_ADDRESS.toString(),
    };
}
