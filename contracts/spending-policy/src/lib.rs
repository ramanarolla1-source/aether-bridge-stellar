# AetherBridge: Sovereign Agentic Rails
> Bridging Hardware Security with Autonomous Machine Payments on Stellar.

AetherBridge is a secure, institutional-grade payment gateway for AI agents. It eliminates the "Bankless Agent" problem by allowing autonomous software to resolve **x402 (Payment Required)** errors using **Soroban-managed Spending Policies** and hardware-anchored trust.

## 🚀 The Vision
AI agents are hitting a "payment wall." Current solutions rely on insecure hot wallets. AetherBridge anchors agent identity in the **Secure Enclave** (Passkeys), ensuring agents can only spend USDC/XLM within strict, on-chain programmable guardrails. This is the foundation for the **National Trust Layer (NTL)**.

## 🛠 Tech Stack & Primitives
- **x402 Protocol:** Enabling programmatic, per-request HTTP payments.
- **Soroban Smart Contracts:** Rust-based spending policies and guardrails.
- **Machine Payments Protocol (MPP):** Handling high-frequency machine-to-machine settlement.
- **Stellar SDK:** Seamless integration with Stellar Testnet for USDC settlement.

## 📜 Smart Contract: Spending Policy
The following Soroban contract (Rust) enforces daily limits on agent spending, preventing drained wallets in the event of an agent logic failure.

```rust
// Paste the Rust code I gave you here
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct SpendingPolicy;

#[contractimpl]
impl SpendingPolicy {
    pub fn check_limit(env: Env, agent: Address, amount: i128) -> bool {
        let daily_limit: i128 = 5000000; // 5.00 USDC (6 decimals)
        let spent_today: i128 = env.storage().instance().get(&agent).unwrap_or(0);
        
        if (spent_today + amount) <= daily_limit {
            env.storage().instance().set(&agent, &(spent_today + amount));
            true
        } else {
            false
        }
    }
}
