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

