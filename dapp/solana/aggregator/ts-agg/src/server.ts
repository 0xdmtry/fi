import { app } from "./app";
import { setWhirlpoolsConfig } from "@orca-so/whirlpools";
import { createSolanaRpc, devnet } from "@solana/kit";

const PORT = process.env.PORT || 8000;

async function startServer() {
    try {
        // Set up Orca SDK config for Devnet
        await setWhirlpoolsConfig("solanaDevnet");

        // Optionally: set default RPC globally (if required)
        const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));
        // await setRpc(rpc); // Only if you use global rpc

        app.listen(PORT, () => {
            console.log(`🚀 Server running on http://localhost:${PORT}`);
        });
    } catch (err) {
        console.error("❌ Failed to initialize SDK:", err);
        process.exit(1);
    }
}

startServer();
