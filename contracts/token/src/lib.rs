use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{U128};
use near_sdk::{env, log, AccountId, near_bindgen, PanicOnDefault};
use std::collections::HashMap;
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    voice_power: HashMap<AccountId, U128>,
    free_tokens: U128

}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(total_supply: U128) -> Self {
        Self::new(
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "FakelessToken".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    #[init]
    pub fn new(
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            voice_power: HashMap::new(),
            free_tokens: total_supply
        };
        this.token.internal_register_account(&env::current_account_id());
        this.token.internal_deposit(&env::current_account_id(), total_supply.into());
        this
    }

    pub fn give_tokens_to(&mut self, amount: U128) 
    {
        assert!(amount.0<=self.free_tokens.0, "There is not enough free tokens");
        if !self.token.accounts.contains_key(&env::signer_account_id()) {
            self.token.internal_register_account(&env::signer_account_id());
        }

        self.token
            .internal_transfer(&env::current_account_id(), &env::signer_account_id(), amount.into(), None);
        self.free_tokens = U128(self.free_tokens.0 - amount.0);
        log!("Gived {:?} tokens to account @{}", amount, &env::signer_account_id());
        log!("Free tokens {}", self.free_tokens.0);
        log!(
            "Full balance {:?}",
            self.token.accounts.get(&env::signer_account_id())
        );
    }

    pub fn stake(&mut self,  amount: U128)
    {
        assert!(amount.0<=self.token.accounts.get(&env::signer_account_id()).unwrap(), "There is not enough tokens on this account");
        self.token
            .internal_transfer(&env::signer_account_id(), &env::current_account_id(), amount.into(), None);
        self.voice_power.insert(env::signer_account_id(), amount);
        log!("Tokens on stake {:?}", self.voice_power.get(&env::signer_account_id()));
        log!(
            "Full balance {:?} on account",
            self.token.accounts.get(&env::signer_account_id())
        );
    }
    pub fn unstake(&mut self,  amount: U128)
    {
        assert!(self.voice_power.contains_key(&env::signer_account_id()), "This account hasn't staked tokens");
        assert!(amount.0<=self.voice_power.get(&env::signer_account_id()).unwrap().0, "You want to unstake more than you have on stake balance");
        self.token
            .internal_transfer(&env::current_account_id(), &env::signer_account_id(), amount.into(), None);
        if amount.0 < self.voice_power.get(&env::signer_account_id()).unwrap().0 
        {
            self.voice_power.insert(env::signer_account_id(), U128(self.voice_power.get(&env::signer_account_id()).unwrap().0 - amount.0));
        } 
        else
        {
            self.voice_power.remove(&env::signer_account_id());
        }

        log!("Tokens on stake {:?}", self.voice_power.get(&env::signer_account_id()));
        log!(
            "Full balance {:?} on account",
            self.token.accounts.get(&env::signer_account_id())
        );
    }
    pub fn get_power(&self, user: AccountId) -> bool
    {
        self.voice_power.contains_key(&user)
    }
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}


