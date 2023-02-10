#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, Get, StorageVersion, UnixTime},
	PalletId,
};
use frame_system::{ensure_root, offchain::SendTransactionTypes, pallet_prelude::*};
pub use pallet::*;
use sp_runtime::{
	traits::{AccountIdConversion, CheckedConversion, Convert, SaturatedConversion},
	KeyTypeId, RuntimeDebug,
};
use sp_staking::SessionIndex;
use sp_std::vec::Vec;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_now: BlockNumberFor<T>) -> Weight {
			T::DbWeight::get().reads(1)
		}

		fn on_finalize(_n: BlockNumberFor<T>) {}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
		if new_index == 0 || new_index == 1 {
			return None
		}
		// TODO fetch the validator lists
		Some(Vec::new())
	}

	fn start_session(start_index: SessionIndex) {}

	fn end_session(end_index: SessionIndex) {}
}

impl<T: Config> pallet_session::historical::SessionManager<T::AccountId, u128> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<(T::AccountId, u128)>> {
		<Self as pallet_session::SessionManager<_>>::new_session(new_index)
			.map(|validators| Vec::new())
	}

	fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<(T::AccountId, u128)>> {
		<Self as pallet_session::SessionManager<_>>::new_session_genesis(new_index)
			.map(|validators| Vec::new())
	}

	fn start_session(start_index: SessionIndex) {
		<Self as pallet_session::SessionManager<_>>::start_session(start_index)
	}

	fn end_session(end_index: SessionIndex) {
		<Self as pallet_session::SessionManager<_>>::end_session(end_index)
	}
}

pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<u128>> for ExposureOf<T> {
	fn convert(validator: T::AccountId) -> Option<u128> {
		Some(0)
	}
}
