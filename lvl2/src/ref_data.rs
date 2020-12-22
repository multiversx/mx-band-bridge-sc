derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Debug)]
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
