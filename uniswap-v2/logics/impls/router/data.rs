use ink::storage::Mapping;
use openbrush::traits::{AccountId, Balance};


#[derive(Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub factory: AccountId,
    pub wnative: AccountId,
    pub withdraw_balance : Mapping<AccountId, (AccountId,Balance)>,    
    pub fee: u64,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            factory: [0u8; 32].into(),
            wnative: [0u8; 32].into(),
            withdraw_balance: Mapping::default(),
            fee:997u64
        }
    }
}
