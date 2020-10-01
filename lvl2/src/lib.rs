
#![no_std]

imports!();

pub mod ref_data;
use ref_data::*;

const USD_TICKER: &[u8] = b"USD";
const U64_1_E9: u64 = 1000000000;
const U64_1_E18: u64 = 1000000000000000000;

#[elrond_wasm_derive::contract(BandBridgeLevel2Impl)]
pub trait BandBridgeLevel2 {

    #[init]
    fn init(&self) {
    }

    #[storage_get("ref")]
    fn get_ref(&self, symbol: Vec<u8>) -> RefData;

    #[storage_set("ref")]
    fn set_ref(&self, symbol: &[u8], ref_data: &RefData);

    #[endpoint]
    fn relay(&self, #[var_args] arguments: VarArgs<MultiArg4<Vec<u8>, u64, u64, u64>>) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "only owner can update price");

        for multi_arg in arguments.into_vec().into_iter() {
            let (symbol, rate, resolve_time, request_id) = multi_arg.into_tuple();
            
            self.set_ref(symbol.as_slice(), &RefData{
                rate,
                resolve_time,
                request_id,
            });            
        }

        Ok(())
    }

    fn get_ref_data(&self, symbol: Vec<u8>) -> SCResult<(BigUint, u64)> {
        if symbol.as_slice() == USD_TICKER {
            Ok((BigUint::from(U64_1_E9), self.get_block_timestamp()))
        } else {
            let ref_data = self.get_ref(symbol);
            require!(!ref_data.is_uninitialized(), "REF_DATA_NOT_AVAILABLE");
            Ok((BigUint::from(ref_data.rate), ref_data.resolve_time))
        }
    }

    #[view(getReferenceData)]
    fn get_reference_data(&self,
        base_symbol: Vec<u8>,
        quote_symbol: Vec<u8>) -> SCResult<MultiResult3<BigUint, u64, u64>> {

        let (base_rate, base_last_update) = sc_try!(self.get_ref_data(base_symbol));
        let (quote_rate, quote_last_update) = sc_try!(self.get_ref_data(quote_symbol));

        let mut rate = base_rate * BigUint::from(U64_1_E18);
        rate /= quote_rate;

        Ok((rate, base_last_update, quote_last_update).into())
    }

    #[view(getReferenceDataBulk)]
    fn get_reference_data_bulk(&self,
        #[var_args] arguments: VarArgs<MultiArg2<Vec<u8>, Vec<u8>>>) 
        -> SCResult<MultiResultVec<MultiResult3<BigUint, u64, u64>>> {

        let mut result_vec = Vec::<MultiResult3<BigUint, u64, u64>>::with_capacity(arguments.len());
        for multi_arg in arguments.into_vec().into_iter() {
            let (base_symbol, quote_symbol) = multi_arg.into_tuple();
            let triple = sc_try!(self.get_reference_data(base_symbol, quote_symbol));
            result_vec.push(triple);
        }

        Ok(result_vec.into())
    }
}
