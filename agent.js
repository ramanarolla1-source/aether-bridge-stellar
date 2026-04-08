// agent.js
import { x402HTTPClient } from "@x402/fetch";
import { createEd25519Signer } from "@x402/stellar";

async function runSovereignAgent() {
    // 1. Initialize the Secure Enclave / Signer
    const signer = createEd25519Signer(process.env.STELLAR_PRIVATE_KEY);
    const client = new x402HTTPClient({ signer });

    console.log("🤖 Agent attempting to access Eons Academic Rail...");

    try {
        // 2. This call will automatically detect a 402 error,
        // sign the authorization, and retry the request.
        const response = await client.fetch("https://api.eons-rail.com/verify/credential_001");

        if (response.ok) {
            const data = await response.json();
            console.log("✅ Access Granted! Data retrieved:", data);
        }
    } catch (error) {
        // 3. Catch spending policy violations or network errors
        console.error("❌ Agent Halted:", error.message);
    }
}

runSovereignAgent();
