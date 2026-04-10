# aether-bridge-stellar
# AetherBridge: Sovereign Agentic Rails

> Bridging Hardware Security with Autonomous Machine Payments on Stellar.

AetherBridge is a secure, institutional-grade payment gateway for AI agents. It eliminates the "Bankless Agent" problem by allowing autonomous software to resolve **x402 (Payment Required)** errors using **Soroban-managed Spending Policies** and hardware-anchored trust.

## 🚀 The Solution: Programmable Guardrails
AI agents often hit a "payment wall." Current solutions rely on insecure hot wallets. AetherBridge anchors agent identity in the **Secure Enclave**, ensuring agents can only spend USDC/XLM within strict, on-chain programmable guardrails.

### 📜 Soroban Spending Policy
Our Rust-based contract enforces granular control over agent spending:
- **Daily Limits:** Prevents drained wallets from logic loops.
- **Whitelist Integration:** Restricts payments to verified service providers.
- **Hardware-Anchored Auth:** Leverages Soroban's native auth for Secure Enclave signatures.

## 🛠 Tech Stack
- Soroban (Rust): Smart contract guardrails for non-custodial agent autonomy.

Stellar SDK (TypeScript): Machine-to-machine settlement using the Stellar network.

x402 Protocol: Standardized HTTP-to-Ledger payment flow for AI-native commerce.

# AetherBridge: Sovereign Agentic Rails
> Bridging Hardware Security with Autonomous Machine Payments on Stellar.

AetherBridge is a secure, institutional-grade payment gateway for AI agents. It eliminates the "Bankless Agent" problem by allowing autonomous software to resolve **x402 (Payment Required)** errors using **Soroban-managed Spending Policies** and hardware-anchored trust.
Integration Flow
Intercept: The agent encounters an HTTP 402 Payment Required header from a service provider.

Verify: AetherBridge checks the Soroban Spending Policy to ensure the request is within the agent's budget.

Sign: The Secure Enclave (Passkey) signs the transaction payload.

Settle: The transaction is submitted to the Stellar Network for near-instant settlement.

Resume: The agent receives the 200 OK and continues its task autonomously.

## 🚀 The Vision
AI agents are hitting a "payment wall." Current solutions rely on insecure hot wallets. AetherBridge anchors agent identity in the **Secure Enclave** (Passkeys), ensuring agents can only spend USDC/XLM within strict, on-chain programmable guardrails. This is the foundation for the **National Trust Layer (NTL)**.

## 🛠 Tech Stack & Primitives
- **x402 Protocol:** Enabling programmatic, per-request HTTP payments.
- **Soroban Smart Contracts:** Rust-based spending policies and guardrails.
- **Machine Payments Protocol (MPP):** Handling high-frequency machine-to-machine settlement.
- **Stellar SDK:** Seamless integration with Stellar Testnet for USDC settlement.

## 📜 Smart Contract: Spending Policy
The following Soroban contract (Rust) enforces daily limits on agent spending, preventing drained wallets in the event of an agent logic failure.

# AetherBridge: Sovereign Agentic Rails

> **Bridging Hardware Security with Autonomous Machine Payments on Stellar.**

AetherBridge is a secure, institutional-grade payment gateway for AI agents. It resolves the "Bankless Agent" problem by allowing autonomous software to resolve **x402 (Payment Required)** errors using **Soroban-managed Spending Policies** and hardware-anchored trust.

---

## 🚀 The Solution: Programmable Guardrails
Current AI agent payment solutions rely on insecure hot wallets. AetherBridge anchors agent identity in the **Secure Enclave** (via Passkeys), ensuring agents can only spend USDC or XLM within strict, on-chain programmable guardrails.

### 🛡️ Soroban Spending Policy
Our Rust-based contract enforces granular control over agent spending, preventing logic-loop drain and unauthorized transfers.

```rust
#[contractimpl]
impl SpendingPolicy {
    pub fn check_and_update(env: Env, agent: Address, amount: i128) {
        agent.require_auth();
        
        let limit = get_daily_limit(&env, &agent);
        let spent = get_spent_today(&env, &agent);
        
        if spent + amount > limit {
            panic!("Spending limit exceeded for today");
        }
        
        update_spent_today(&env, &agent, spent + amount);
    }
}
Agent Workflow (x402 Integration)
Our agent catches the HTTP 402 error and automatically retries the request after authorizing a Stellar transaction via AetherBridge.

Agent Request: Calls premium API (e.g., Eons Academic Rail).

402 Error: Server responds with Payment Required and Stellar payment headers.

Note: AetherBridge utilizes the Stellar-MPP-SDK to manage these machine-to-machine payment flows, moving beyond human-centric dashboards for true agentic autonomy.

AetherBridge Check: Verifies request against Soroban Spending Policy.

Hardware Auth: Passkey/Secure Enclave signs the authorization entry.

Settlement: USDC settles on Stellar; Agent receives data.

## 🛠️ Tech Stack: Deep Stellar Integration

AetherBridge isn't just a wrapper; it is a native extension of the Stellar ecosystem:

- **Soroban Native Auth Framework:** Unlike EVM-based `msg.sender` patterns, we utilize Soroban’s explicit `require_auth()` and `require_auth_for_args()`. This allows our contracts to verify that a transaction was authorized by the specific hardware-anchored identity (Passkey) without managing complex signature logic in-contract.
- **Smart Account Guardrails:** We move beyond simple "multisig." AetherBridge implements a non-custodial **Smart Account** pattern where the agent is the primary actor, but the **Spending Policy** contract acts as an immutable guardrail. This ensures that even if an agent's logic loops, it cannot drain the treasury.
- **Stellar Asset Contract (SAC):** Built-in support for **USDC** and **XLM** via the native asset bridge, ensuring institutional-grade settlement with sub-5 second finality.

## 🤖 The Paradigm: Non-Custodial Agent Autonomy

We are moving AI agents from "Read-Only" to "Transact-Ready." 

In the AetherBridge model, the agent is **autonomous but not ungoverned**. By leveraging Stellar’s Protocol 23 improvements (specifically parallel execution and cleaner auth), we enable:
1. **Passkey Verification:** Agents sign with device-bound credentials (WebAuthn/Secp256r1), eliminating seed phrases.
2. **Policy-Based Authorization:** Authorization is data-driven. The agent identifies *who* it is, and the **Smart Account Guardrail** dictates *what* it is allowed to do based on the current context (e.g., "Pay for this API inference up to 5 USDC").

Machine-to-Machine (M2M) logic flow
## 🔄 The AetherBridge M2M Flow

AetherBridge operates as a seamless "Interceptor" within the agent's network stack, following the standardized **x402 (Payment Required)** lifecycle:

1. **402 Challenge:** The AI agent makes an HTTP request to a service provider (e.g., an LLM API). The server responds with an `HTTP 402 Payment Required` header, specifying the amount in USDC and the destination address.
2. **Policy Verification:** The AetherBridge SDK intercepts this 402 challenge. It queries the **Soroban Spending Policy** contract on Stellar to verify if the payment is within the agent's pre-approved "Smart Account Guardrails."
3. **Hardware Signature:** If the policy check passes, the SDK triggers a signature from the **Secure Enclave (Passkey)**. No seed phrases are ever exposed to the agent's logic.
4. **On-Chain Settlement:** The signed authorization is submitted to a **Stellar Facilitator**. The payment settles on the ledger in ~5 seconds with sub-cent fees.
5. **Resource Delivery:** AetherBridge retries the original request with the transaction proof attached. The service provider validates the proof and returns the `200 OK` response.
