//! Implements support for the AssetRegistry module.

use super::primitives::{
    asset::Asset,
    did::Did,
};
use crate::frame::{
    system::System,
    Call,
};
use codec::Encode;
/// Module name
pub const MODULE: &str = "AssetRegistry";

pub type RegistryId = u32;
pub type AssetIndex = u64;
pub type Moment = u64;

pub trait AssetRegistry: System {}
mod calls {
    pub const CREATE_REGISTRY: &str = "create_registry";
    pub const CREATE_ASSET: &str = "create_asset";
}
/// events
#[allow(unused)]
pub mod events {
    pub const REGISTRY_CREATED: &str = "RegistryCreated";
    pub const ASSET_CREATED: &str = "AssetCreated";
}

#[derive(Encode)]
pub struct CreateRegistryArgs {
    owner_did: Did,
}

pub fn create_registry(owner_did: Did) -> Call<CreateRegistryArgs> {
    Call::new(
        MODULE,
        calls::CREATE_REGISTRY,
        CreateRegistryArgs { owner_did },
    )
}
#[derive(Encode)]
pub struct CreateAssetArgs {
    registry_id: RegistryId,
    asset: Asset<Moment, AssetIndex>,
}

pub fn create_asset(
    registry_id: RegistryId,
    asset: Asset<Moment, AssetIndex>,
) -> Call<CreateAssetArgs> {
    Call::new(
        MODULE,
        calls::CREATE_ASSET,
        CreateAssetArgs { registry_id, asset },
    )
}
