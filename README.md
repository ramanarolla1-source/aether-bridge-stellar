![AetherBridge Sovereign Agentic Rails](https://github.com/user-attachments/assets/00d514c0-b65e-48ca-9e7d-fab7bbf7af81)



# AetherBridge: Sovereign Agentic Rails

> **Bridging Hardware Security with Autonomous Machine Payments on Stellar.**

AetherBridge is a secure, institutional-grade payment gateway for AI agents. It eliminates the "Bankless Agent" problem by allowing autonomous software to resolve **x402 (Payment Required)** errors using **Soroban-managed Smart Account Guardrails** and hardware-anchored trust.

---

## 🚀 The Solution: Programmable Guardrails
Current AI agent payment solutions often rely on insecure "hot wallets" that are vulnerable to logic loops or unauthorized drains. AetherBridge introduces a non-custodial **Smart Account** pattern where the agent is autonomous but governed by on-chain policies anchored in the **Secure Enclave** (via Passkeys).

### 🛡️ Soroban Smart Account Policy
Our Rust-based contract implements granular, stateful guardrails. This ensures that agents can only spend assets (USDC/XLM) within pre-approved parameters, even if the agent's logic is compromised.

```rust
#[contractimpl]
impl SpendingPolicy {
    pub fn check_and_update(env: Env, agent: Address, amount: i128) {
        // Utilizing Soroban Native Auth for hardware-anchored verification
        agent.require_auth();
        
        let limit = get_daily_limit(&env, &agent);
        let spent = get_spent_today(&env, &agent);
        
        if spent + amount > limit {
            panic!("Spending limit exceeded: Smart Account Guardrail triggered.");
        }
        
        update_spent_today(&env, &agent, spent + amount);
    }
}
Soroban Technical Specification: AetherBridge
1. Non-Custodial Agent Authentication
AetherBridge utilizes Soroban’s Built-in Authorization Framework (require_auth) to manage agentic transactions without centralized key storage.

Mechanism: Agents utilize unique ed25519 identities generated at the edge.

Security: Instead of holding keys on a server, AetherBridge leverages Soroban’s Account Abstraction capabilities, allowing smart contracts to verify agent signatures directly on-chain before executing an x402 settlement.

2. Automated x402 Intercepts on Soroban
We have implemented the x402 Payment Required logic as a native Soroban contract.

Logic: When an agent requests a high-value resource, the contract "intercepts" the call, verifies the pre-signed authorization, and executes the Token Transfer in a single atomic transaction.

Efficiency: This eliminates the need for multi-step "Approve/TransferFrom" patterns, reducing gas (resource) consumption on the network.

3. State TTL & Persistent Rails
To ensure AetherBridge remains "always-on" for long-running industrial cycles (like the fuel trade use case):

Storage: We utilize Persistent Storage for agent reputation and ledger snapshots.

Lifecycle: The contract includes an automated extend_ttl function, ensuring that the "Sovereign Rails" do not archive during periods of low activity between batch settlements.

4. Autonomous Guardrails
Every agentic action is governed by a Guardrail Contract that restricts transactions based on:

Velocity Limits: Maximum spend per ledger.

Destination Whitelisting: Ensuring fees are only redirected to authorized service providers.


🛠️ Tech Stack: Deep Stellar Integration
Soroban Native Auth Framework: Unlike generic signature checks, we utilize native require_auth() to leverage Stellar’s Protocol 23 improvements, supporting Passkeys (Secp256r1) directly at the protocol level.

Stellar Machine Payments Protocol (MPP): Fully compliant with the x402 open standard for internet-native payments.

Cross-Asset Settlement: Native support for USDC via the Stellar Asset Contract (SAC), providing institutional finality in <5 seconds.

🔄 The AetherBridge M2M Flow
AetherBridge operates as a seamless "Interceptor" within the agent's network stack, following the standardized x402 lifecycle:

402 Challenge: The AI agent hits a paid resource. The server responds with an HTTP 402 Payment Required header.

Policy Verification: The AetherBridge SDK intercepts the challenge and queries the Soroban Spending Policy to ensure the transaction is authorized.

Hardware Signature: The Secure Enclave (Passkey) signs the transaction. No private keys are ever exposed to the agent’s execution environment.

On-Chain Settlement: The signed payload is submitted to Stellar for near-instant, low-cost settlement ($0.00001 fees).

Resource Access: The agent retries the request with the transaction proof, receiving the 200 OK and continuing its task.

📂 Repository Structure
/contracts: Rust-native Soroban contracts for Smart Account Guardrails.

/agent-sdk: TypeScript interceptor logic for automated x402 handling.

/examples: Demo scripts showing an "Autonomous Pay-per-Inference" workflow.

AetherBridge is a purpose-built implementation of the AetherUX logic, optimized for the Stellar Machine Payments Protocol (MPP) and the emerging Agentic Economy.

One Pager: https://docs.google.com/document/d/1NSiYEb2IdbknaQzjBLypy02216JuLw91gnA5v256ICE/edit?usp=sharing
Demo Video: https://youtu.be/szuxz5TDkIA


🧮 The Economic Decision Model (EDM)AetherBridge implements a deterministic logic for agentic spending. When an agent encounters an HTTP 402 (Payment Required), AetherBridge evaluates the transaction through a three-tier mathematical filter:Utility Threshold: Is the cost of the data ($C$) less than the projected value of the task ($V$)?Gas Optimization: Should the transaction be routed through a Fee Facilitator (for $0 XLM gas) based on current network congestion?Settlement Latency: AetherBridge uses Atomic Soroban Transactions to ensure that the payment and the data-key exchange happen in a single, trustless ledger entry, eliminating the "counterparty risk" usually found in M2M commerce.
