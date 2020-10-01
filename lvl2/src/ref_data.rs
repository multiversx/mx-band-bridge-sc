use elrond_wasm::elrond_codec::*;

#[derive(PartialEq, Debug)]
pub struct RefData {
    /// USD-rate, multiplied by 1e9.
    pub rate: u64,

    /// UNIX epoch when data is last resolved.
    pub resolve_time: u64,

    /// BandChain request identifier for this data.
    pub request_id: u64,
}

impl RefData {
    pub fn is_uninitialized(&self) -> bool {
        self.resolve_time == 0
    }
}

// Will be auto-generated at some point in the future.
impl Encode for RefData {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.rate.dep_encode_to(dest)?;
        self.resolve_time.dep_encode_to(dest)?;
        self.request_id.dep_encode_to(dest)?;
        Ok(())
    }
}

// Will be auto-generated at some point in the future.
impl Decode for RefData {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(RefData {
            rate: u64::dep_decode(input)?,
            resolve_time: u64::dep_decode(input)?,
            request_id: u64::dep_decode(input)?,
        })
    }

    // this allows the contract to deserialize empty storage
    fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        if input.remaining_len() == 0 {
            // nothing in storage 
            Ok(RefData {
                rate: 0,
                resolve_time: 0,
                request_id: 0,
            })
        } else {
            let result = Self::dep_decode(input)?;
            if input.remaining_len() > 0 {
                return Err(DecodeError::InputTooLong);
            }
            Ok(result)
        }
    }
}
