use solana_sdk::{signature::Keypair, signer::Signer};
use solana_sdk::pubkey::Pubkey;
use bip39::{Mnemonic, MnemonicType, Seed, Language};

#[derive(Debug, Clone, PartialEq)]
pub enum JourneyOption {
    GenerateKeypair,
    GenerateMnemonic,
    ClaimFromFaucet,
}

#[derive(Debug)]
struct MenuItem {
    name: JourneyOption,
}

pub struct KeypairJourney {
    current_keypair: Option<Keypair>,
    mnemonic_phrase: Option<String>,
    selected_option: Option<JourneyOption>,
    faucet_request_status: Option<String>,
    save_status: Option<String>,
}

impl KeypairJourney {
    pub fn new() -> Self {
        Self {
            current_keypair: None,
            mnemonic_phrase: None,
            selected_option: None,
            faucet_request_status: None,
            save_status: None,
        }
    }

    pub fn select_option(&mut self, option: JourneyOption){
        self.selected_option = Some(option);
    }

    pub fn get_selected_option(&self) -> Option<&JourneyOption>{
        self.selected_option.as_ref()
    }

    pub fn generate_keypair(&mut self) -> Result<()>{
        let keypair = Keypair::new();
        self.current_keypair = Some(keypair);
        self.mnemonic_phrase = None;
        Ok(())
    }

    pub fn generate_mnemonic_from_keypair(&mut self) -> Result<()>{
        let keypair = self.current_keypair.as_ref().unwrap();
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        mnemonic.phrase()
    }

    pub fn request_airdrop(){}

    pub fn check_balance(){}

    pub fn get_mnemonic_from_keypair(&self) -> Option<&str>{
        self.mnemonic_phrase.as_deref()
    }
}
