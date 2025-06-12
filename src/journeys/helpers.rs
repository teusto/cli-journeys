use solana_sdk::{signature::Keypair, signer::Signer, pubkey::Pubkey};
use bip39::{Language, Mnemonic};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

#[derive(Debug)]
pub enum JourneyOutput {
    Keypair(Keypair),
    Mnemonic(Mnemonic),
    BalanceChecker(String),
    None,
}

impl Default for JourneyOutput {
    fn default() -> Self {
        JourneyOutput::None
    }
}

pub fn generate_keypair() -> Keypair {
    Keypair::new()
}

#[inline]
pub fn as_keypair_output() -> JourneyOutput {
    let kp = Keypair::new();
    JourneyOutput::Keypair(kp)
}

#[inline]
pub fn as_mnemonic_output() -> JourneyOutput {
    let mnemonic = Mnemonic::generate(12).unwrap();
    JourneyOutput::Mnemonic(mnemonic)
}

pub fn as_balance_checker() -> JourneyOutput {
    let sample_account = "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg";
    let networks = [
        ("Mainnet", "https://api.mainnet-beta.solana.com"),
        ("Testnet", "https://api.testnet.solana.com"),
        ("Devnet", "https://api.devnet.solana.com"),
    ];

    let mut results = format!("Checking for: {}\n", sample_account);
    results.push_str("-------------------------------\n\n");

    match Pubkey::from_str(sample_account) {
        Ok(pubkey) => {
            for(network_name, endpoint) in networks.iter() {
                results.push_str(&format!("Network: {}\n", network_name));

                let client = RpcClient::new(endpoint.to_string());

                match client.get_balance(&pubkey) {
                    Ok(balance) => {
                        let sol_balance = balance as f64 / 1_000_000_000.0;
                        results.push_str(&format!("Balance: {} SOL ({} lamports)\n", sol_balance, balance));
                    },
                    Err(err) => {
                        results.push_str(&format!("Error while querying balance: {}\n", err));
                    }
                }
                results.push_str("\n");
            }
        },
        Err(_) => {
            results.push_str("Invalid Solana address format\n");
        }
    }
    JourneyOutput::BalanceChecker(results)
}

pub fn run_journey(journey_name: &str) -> JourneyOutput {
    match journey_name {
        "Keypair Generation" => {
            as_keypair_output()
        }
        "Mnemonics Generation" => {
            as_mnemonic_output()
        }
        "Balance Checker" => {
            as_balance_checker()
        }
        _ => {
            JourneyOutput::None
        } 
    }
}