#![no_std]

imports!();

const USD_TICKER: &[u8] = b"USD";

#[elrond_wasm_derive::callable(StdReferenceInterface)]
pub trait StdReferenceInterface {
    #[callback(set_price_callback)]
    fn get_reference_data(
        &self,
        base_symbol: BoxedBytes,
        quote_symbol: BoxedBytes,
        #[callback_arg] cb_base_symbol: BoxedBytes,
    ) -> SCResult<MultiResult3<BigUint, u64, u64>>;
}

#[elrond_wasm_derive::contract(SimplePriceDbImpl)]
pub trait SimplePriceDb {
    #[init]
    fn init(&self) {}

    /// Oracle address.
    #[view(getStdReference)]
    #[storage_get("std_reference")]
    fn get_std_reference(&self) -> Address;

    #[storage_set("price")]
    fn set_price(&self, base_symbol: &[u8], price: &BigUint);

    #[storage_get("price")]
    fn get_price(&self, base_symbol: &[u8]) -> BigUint;

    #[storage_set("std_reference")]
    fn set_std_reference(&self, address: &Address);

    #[endpoint(setStdReference)]
    fn set_std_reference_endpoint(&self, address: &Address) -> SCResult<()> {
        only_owner!(self, "only owner can set STD reference");
        self.set_std_reference(address);
        Ok(())
    }

    #[view(getPrice)]
    fn get_price_endpoint(&self, base_symbol: &[u8]) -> SCResult<BigUint> {
        let price = self.get_price(base_symbol);
        require!(price > 0, "PRICE_NOT_SET");
        Ok(price)
    }

    #[endpoint(savePrice)]
    fn save_price(&self, base_symbol: BoxedBytes) -> SCResult<()> {
        only_owner!(self, "only owner can save price");
        let std_reference = self.get_std_reference();
        let std_ref = contract_proxy!(self, &std_reference, StdReferenceInterface);
        std_ref.get_reference_data(base_symbol.clone(), USD_TICKER.into(), base_symbol);
        Ok(())
    }

    /// The MultiResult3 from the other contract arrives in the callback as a MultiArg3.
    #[callback]
    fn set_price_callback(
        &self,
        result: AsyncCallResult<MultiArg3<BigUint, u64, u64>>,
        #[callback_arg] cb_base_symbol: BoxedBytes,
    ) {
        match result {
            AsyncCallResult::Ok(result) => {
                let rate = result.into_tuple().0;
                self.set_price(cb_base_symbol.as_slice(), &rate);
            }
            AsyncCallResult::Err(_) => {
                self.set_price(cb_base_symbol.as_slice(), &BigUint::zero());
            }
        }
    }
}
