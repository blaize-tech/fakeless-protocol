use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use std::collections::HashSet;
use near_sdk::serde::{Serialize, Deserialize};
use std::convert::{TryInto, TryFrom};

near_sdk::setup_alloc!();

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct News
{
    pub id: u32,
    pub hash_head: String,
    pub hash_body: String,
    pub uri: String,
    pub like: u64,
    pub dislike: u64,
    pub voted: HashSet<String>, 
    pub creator: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NewsStorage {
    news: Vec<News>,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl NewsStorage {
    pub fn add(&mut self, hash_head: String, hash_body: String, uri: String) 
    {
        self.news.push(News 
            {
                id: (self.news.len() as u32) + 1, 
                hash_head, 
                hash_body, 
                uri,
                like: 0,
                dislike: 0,
                voted: HashSet::new(),
                creator: env::signer_account_id(),
            }
        );
    }

    pub fn get_all(&self)-> Vec<News>
    {
        self.news.clone()
    }

    pub fn get_by_index(&self, index: usize)-> News
    {
        self.news[index].clone()
    }

    pub fn upvote(&mut self, index: usize) 
    {
        assert!(index < self.news.len());
        assert!(!self.news[index].voted.contains(&env::signer_account_id()));
        self.news[index].like = self.news[index].like.saturating_add(1);
        self.news[index].voted.insert(env::signer_account_id().clone());
    }

    pub fn downvote(&mut self, index: usize) 
    {
        assert!(index < self.news.len());
        assert!(!self.news[index].voted.contains(&env::signer_account_id()));
        self.news[index].dislike = self.news[index].dislike.saturating_add(1);
        self.news[index].voted.insert(env::signer_account_id().clone());
    }
    #[init]
    pub fn new_default_meta() -> Self {
        Self::new(
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Fakeless".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(metadata: NFTContractMetadata) -> Self {
        metadata.assert_valid();
        Self {
            news: Vec::<News>::new(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                env::signer_account_id().try_into().unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        index: usize,
    ) -> Token {
        assert!(index < self.news.len());
        assert!(self.news[index].like>=100, "There are not enough likes to publish this news");
        assert_eq!(self.news[index].creator, env::signer_account_id().to_string(), "You are not a creator");
        self.tokens.owner_id = env::signer_account_id().try_into().unwrap();
        self.tokens.mint(self.news[index].id.to_string().clone(), ValidAccountId::try_from(env::signer_account_id().to_string().clone()).unwrap() , Some(TokenMetadata{
            title:Some(self.news[index].hash_head.clone()),
            description:Some(self.news[index].hash_body.clone()),
            media:Some("https://iat.kpi.ua/wp-content/uploads/2019/10/news-3.jpg".to_string()),
            copies:Some(1), 
            media_hash: None,
            issued_at: None, 
            expires_at: None, 
            starts_at: None, 
            updated_at: None, 
            extra: None, 
            reference: None, 
            reference_hash: None }))
    }
}

near_contract_standards::impl_non_fungible_token_core!(NewsStorage, tokens);
near_contract_standards::impl_non_fungible_token_approval!(NewsStorage, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(NewsStorage, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for NewsStorage {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
