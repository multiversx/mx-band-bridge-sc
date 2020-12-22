#![no_std]

imports!();

#[elrond_wasm_derive::contract(BandBridgeLevel1Impl)]
pub trait BandBridgeLevel1 {
    #[init]
    fn init(&self) {}

    #[view(getPrice)]
    #[storage_get("price")]
    fn get_price_data(&self, symbol: Vec<u8>) -> Vec<u8>;

    #[storage_set("price")]
    fn set_price_data(&self, symbol: &[u8], price_data: &[u8]);

    #[endpoint(updataPrice)]
    fn update_price(&self, symbol: Vec<u8>, price_data: Vec<u8>) -> SCResult<()> {
        require!(
            self.get_caller() == self.get_owner_address(),
            "only owner can update price"
        );

        self.set_price_data(symbol.as_slice(), price_data.as_slice());

        Ok(())
    }
}
