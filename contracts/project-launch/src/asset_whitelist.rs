//! Asset whitelist for protected assets.
//!
//! Trust model: only admin-verified callers may mutate the whitelist; all external callers are untrusted.
//! Enforcement invariant: no contribution to a whitelisted asset may proceed unless the contributor's
//! KYC tier meets or exceeds the asset's required tier at the time of the transaction.
//! Atomicity invariant: any failed whitelist write leaves on-chain state unchanged.

use soroban_sdk::{Address, Env};

use shared::errors::Error;
use shared::events::{ASSET_WHITELIST_REMOVED, ASSET_WHITELIST_SET};
use shared::types::KycTier;

pub struct AssetWhitelist;

impl AssetWhitelist {
    fn storage_key(asset: &Address) -> (crate::DataKey, Address) {
        (crate::DataKey::AssetWhitelist, asset.clone())
    }

    /// Set or update the required KYC tier for a protected asset.
    ///
    /// # Arguments
    ///
    /// * `env` - Contract environment.
    /// * `admin` - Admin address signing the transaction.
    /// * `asset` - Asset address being protected.
    /// * `required_tier` - Required KYC tier for this asset.
    ///
    /// # Errors
    ///
    /// * `Error::NotInit` if the contract has not been initialized.
    /// * `Error::Unauthorized` if the caller is not the stored admin.
    /// * `Error::InvalidKycTier` if `required_tier` is zero.
    pub fn set(env: Env, admin: Address, asset: Address, required_tier: KycTier) -> Result<(), Error> {
        if required_tier == 0 {
            return Err(Error::InvalidKycTier);
        }

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&crate::DataKey::Admin)
            .ok_or(Error::NotInit)?;

        if stored_admin != admin {
            return Err(Error::Unauthorized);
        }

        admin.require_auth();
        env.storage()
            .instance()
            .set(&Self::storage_key(&asset), &required_tier);
        env.events()
            .publish((ASSET_WHITELIST_SET,), (asset, required_tier));

        Ok(())
    }

    /// Remove an asset from the protected asset whitelist.
    ///
    /// # Arguments
    ///
    /// * `env` - Contract environment.
    /// * `admin` - Admin address signing the transaction.
    /// * `asset` - Asset address to remove.
    ///
    /// # Errors
    ///
    /// * `Error::NotInit` if the contract has not been initialized.
    /// * `Error::Unauthorized` if the caller is not the stored admin.
    /// * `Error::NotFound` if the asset is not currently whitelisted.
    pub fn remove(env: Env, admin: Address, asset: Address) -> Result<(), Error> {
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&crate::DataKey::Admin)
            .ok_or(Error::NotInit)?;

        if stored_admin != admin {
            return Err(Error::Unauthorized);
        }

        admin.require_auth();
        let key = Self::storage_key(&asset);
        if !env.storage().instance().has(&key) {
            return Err(Error::NotFound);
        }

        env.storage().instance().remove(&key);
        env.events().publish((ASSET_WHITELIST_REMOVED,), asset);
        Ok(())
    }

    /// Get the required KYC tier for a protected asset.
    ///
    /// Returns `None` when the asset is not whitelisted.
    pub fn get_required_tier(env: &Env, asset: &Address) -> Option<KycTier> {
        env.storage().instance().get(&Self::storage_key(asset))
    }

    /// Validate a contributor's KYC tier against the asset's whitelist requirement.
    ///
    /// # Arguments
    ///
    /// * `env` - Contract environment.
    /// * `asset` - Asset address being contributed.
    /// * `user_tier` - Contributor's current KYC tier.
    ///
    /// # Errors
    ///
    /// * `Error::InvalidKycTier` if the stored asset requirement is invalid.
    /// * `Error::KycTierInsufficient` if the contributor's tier is below the asset requirement.
    pub fn validate_asset_kyc(env: &Env, asset: &Address, user_tier: KycTier) -> Result<(), Error> {
        if let Some(required_tier) = Self::get_required_tier(env, asset) {
            if required_tier == 0 {
                return Err(Error::InvalidKycTier);
            }
            if user_tier < required_tier {
                return Err(Error::KycTierInsufficient);
            }
        }
        Ok(())
    }
}
