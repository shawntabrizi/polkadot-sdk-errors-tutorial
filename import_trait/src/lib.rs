//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet]
pub mod pallet {
	use super::*;
	use frame::traits::fungible::Inspect;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type NativeBalance: Inspect<Self::AccountId>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}

impl<T: Config> Pallet<T> {
	pub fn get_total_balance(who: T::AccountId) -> BalanceOf<T> {
		// TODO: Comment/Uncomment this line.
		use frame::traits::fungible::Inspect;
		T::NativeBalance::total_balance(&who)
	}
}
