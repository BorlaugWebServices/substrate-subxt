use super::did::Did;
use codec::{
    Decode,
    Encode,
};
use frame_support::dispatch::Vec;
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, RuntimeDebug)]
pub struct Asset<Timestamp, AssetIndex> {
    pub asset_id: AssetIndex,
    pub created_by: Did,
    pub total_shares: u64,
    pub lessees: Vec<Lessee>,
    pub residual_value: u64,
    pub status: AssetStatus,
    pub serial_no: Vec<u8>,
    pub purchase_value: u64,
    pub aquired_date: Timestamp,
    pub number: Vec<u8>,
    pub name: Vec<u8>,
}
#[derive(Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, RuntimeDebug)]
pub struct Lessee {
    pub did: Did,
    pub shares: u64,
}
#[derive(Encode, Decode, PartialOrd, Ord, PartialEq, Eq, Clone, RuntimeDebug)]
pub enum AssetStatus {
    Draft,
    Active,
    InActive,
}
impl Default for AssetStatus {
    fn default() -> Self {
        AssetStatus::Active
    }
}
