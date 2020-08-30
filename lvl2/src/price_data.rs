use elrond_wasm::elrond_codec::*;

#[derive(PartialEq, Debug)]
pub struct PriceData {
    pub price: u64,
    pub multiplier: u64,
    pub last_update: u64,
}

impl Encode for PriceData {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.price.dep_encode_to(dest)?;
        self.multiplier.dep_encode_to(dest)?;
        self.last_update.dep_encode_to(dest)?;
        Ok(())
	}
}

impl Decode for PriceData {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(PriceData {
            price: u64::dep_decode(input)?,
            multiplier: u64::dep_decode(input)?,
            last_update: u64::dep_decode(input)?,
        })
    }
}
