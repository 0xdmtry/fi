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
    PublicKey,
    TransactionInstruction,
    VersionedTransaction,
    TransactionMessage,
    Keypair,
} from "@solana/web3.js";
import {requestAtaCreationFromWalletor} from "./ataService";
import {requestSignatureFromWalletor} from "./walletorService";
import {SwapRequest, SwapUnsignedResponse} from "../payloads/swap";
import {AccountRole, Address} from "@solana/kit";
import Decimal from "decimal.js";

// Setup RPC
const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));

const POOL_ADDRESS = address("3KBZiL2g8C7tiJ32hTv5v3KM7aK9htpqTw4cTXz1HvPt");
const WSOL_MINT = address("So11111111111111111111111111111111111111112");
const WSOL_DECIMALS = 9;

function toAccountMeta(account: { address: Address; role: AccountRole }): {
    pubkey: PublicKey,
    isSigner: boolean,
    isWritable: boolean
} {
    const isSigner = account.role === AccountRole.READONLY_SIGNER || account.role === AccountRole.WRITABLE_SIGNER;
    const isWritable = account.role === AccountRole.WRITABLE || account.role === AccountRole.WRITABLE_SIGNER;
    console.log(`→ account: ${account.address} | role: ${account.role} | isSigner: ${isSigner} | isWritable: ${isWritable}`);
    return {
        pubkey: new PublicKey(account.address),
        isSigner,
        isWritable,
    };
}

export async function buildUnsignedSwapTransaction(req: SwapRequest): Promise<SwapUnsignedResponse> {
    console.log("🔧 buildUnsignedSwapTransaction");
    const {inputMint, amount, slippage, userPublicKey} = req;
    console.log("📥 inputMint:", inputMint);
    console.log("💰 amount:", amount);
    console.log("📉 slippage:", slippage);
    console.log("👤 userPublicKey:", userPublicKey);

    // STEP 1: Pre-create ATA (if needed)
    try {
        const ataResult = await requestAtaCreationFromWalletor({
            user_id: "3c75c313-48b6-4728-9397-3e8004255875",
            token_mint: inputMint,
        });
        console.log("✅ ATA created:", ataResult);
    } catch (err) {
        console.error("❌ Failed to create ATA via Walletor:", err);
        throw err;
    }

    // STEP 2: Convert amount + slippage
    const inputAmountDecimal = new Decimal(amount);
    const inputAmountRaw = BigInt(
        inputAmountDecimal.mul(Decimal.pow(10, WSOL_DECIMALS)).toFixed(0)
    );
    const slippageBps = Math.floor(slippage * 100);
    console.log("🧮 inputAmountRaw:", inputAmountRaw);
    console.log("🎯 slippageBps:", slippageBps);

    // STEP 3: Build swap instructions
    const {instructions} = await swapInstructions(
        rpc,
        {
            inputAmount: inputAmountRaw,
            mint: WSOL_MINT, // Make sure this is WSOL as SPL, not native SOL
        },
        POOL_ADDRESS,
        slippageBps,
        {
            address: address(userPublicKey),
            signAndSendTransactions: () => {
                throw new Error("signAndSendTransactions is not supported in MPC flow");
            },
        }
    );

    console.log("🧾 Instructions fetched from Orca:");
    instructions.forEach((ix, i) => {
        if (!ix || !ix.accounts) {
            console.error("❌ Invalid instruction format:", ix);
            return;
        }

        console.log(`📦 Instruction[${i}]`);
        console.log(`  ↪ Program: ${ix.programAddress}`);
        ix.accounts.forEach((acc, j) => {
            console.log(`    🔹 Account[${j}]: ${acc.address} | role: ${acc.role}`);
        });
    });

    const blockhash = (await rpc.getLatestBlockhash().send()).value.blockhash;
    console.log("⛓️ Blockhash:", blockhash);

    // STEP 4: Convert to TransactionInstruction[]
    const realInstructions: TransactionInstruction[] = instructions.map((ix, idx) => {

        if (!ix || !ix.accounts || !ix.data) {
            console.error("❌ Invalid instruction format:", ix);
            throw new Error(`Invalid instruction format at index ${idx}`);
        }

        const keys = ix.accounts.map(toAccountMeta);
        return new TransactionInstruction({
            programId: new PublicKey(ix.programAddress),
            keys,
            data: Buffer.from(ix.data),
        });
    });

    // STEP 5: Compile Message
    const message = new TransactionMessage({
        payerKey: new PublicKey(userPublicKey),
        recentBlockhash: blockhash,
        instructions: realInstructions,
    }).compileToV0Message();

    console.log("📩 Compiled V0 message:");
    console.log("  🔐 Required signers:", message.header.numRequiredSignatures);
    console.log("  📚 Static account keys:");
    message.staticAccountKeys.forEach((key, idx) => {
        const label = idx < message.header.numRequiredSignatures ? "SIGNER" : "NON-SIGNER";
        console.log(`    ${label} [${idx}]: ${key.toBase58()}`);
    });

    // STEP 6: Prepare transaction object
    const tx = new VersionedTransaction(message);

    // STEP 7: Optional — Sign locally if you generated ephemeral keypairs
    // Example placeholder (commented out):
    // const tempKeypair = Keypair.generate(); // e.g. for WSOL temp account
    // tx.sign([tempKeypair]);
    // console.log("✍️ Locally signed with ephemeral keypair:", tempKeypair.publicKey.toBase58());

    // STEP 8: Print partially signed transaction state
    const numSignatures = tx.signatures.filter(sig => !sig.every(b => b === 0)).length;
    console.log(`🧾 Transaction signature slots:`);
    tx.signatures.forEach((sig, i) => {
        const isSigned = !sig.every(b => b === 0);
        const status = isSigned ? "✅ Signed" : "⛔ Not Signed";
        console.log(`  Slot[${i}] ${status}`);
    });
    console.log(`  👉 Total signed: ${numSignatures}/${message.header.numRequiredSignatures}`);

    // STEP 9: Serialize and send to Walletor for final signature
    const txBase64 = Buffer.from(tx.serialize()).toString("base64");
    console.log("📦 Transaction base64 (pre-MPC):", txBase64.slice(0, 120) + "...");

    await requestSignatureFromWalletor({
        user_id: "3c75c313-48b6-4728-9397-3e8004255875", // TEMPORARY
        transaction_base64: txBase64,
    });

    console.log("✅ Walletor signature completed");

    return {
        transactionBase64: txBase64,
        pool: POOL_ADDRESS.toString(),
    };
}
