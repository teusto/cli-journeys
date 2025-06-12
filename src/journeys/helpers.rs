use solana_sdk::{signature::Keypair, signer::Signer};
use bip39::{Language, Mnemonic};

#[derive(Debug)]
pub enum JourneyOutput {
    Keypair(Keypair),
    Mnemonic(Mnemonic),
    RentCalculator(String),
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
pub fn as_mnemonic_output(mnemonic: String) -> JourneyOutput {
    let mnemonic = Mnemonic::generate(12).unwrap();
    JourneyOutput::Mnemonic(mnemonic)
}

pub fn run_journey(journey_name: &str) -> JourneyOutput {
    match journey_name {
        "Keypair Generation" => {
            as_keypair_output()
        }
        "Mnemonics Generation" => {
            as_mnemonic_output("example mnemonic phrase".into())
        }
        _ => {
            JourneyOutput::None
        } 
    }
}