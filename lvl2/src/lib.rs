
#![no_std]

imports!();

pub mod price_data;
use price_data::*;

#[elrond_wasm_derive::contract(BandBridgeLevel2Impl)]
pub trait BandBridgeLevel2 {

    #[init]
    fn init(&self) {
    }

    #[view(getPrice)]
    #[storage_get("price")]
    fn get_price_data(&self, symbol: Vec<u8>) -> PriceData;

    #[storage_set("price")]
    fn set_price_data(&self, symbol: &[u8], price_data: &PriceData);

    #[endpoint(updataPrice)]
    fn update_price(&self, symbol: Vec<u8>, price: u64, multiplier: u64) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "only owner can update price");

        self.set_price_data(symbol.as_slice(), &PriceData{
            price,
            multiplier,
            last_update: self.get_block_nonce(),
        });

        Ok(())
    }
    
}
