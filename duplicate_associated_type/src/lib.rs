pub mod pallet_balances {
	pub trait Config {
		type Balance;
	}
}

pub mod pallet_assets {
	use super::*;

	pub trait Config: pallet_balances::Config {
		type AssetId;
		type Balance;
	}

	pub struct Pallet<T>(core::marker::PhantomData<T>);

	impl<T: Config> Pallet<T> {
		pub fn total_issuance() -> <T as pallet_balances::Config>::Balance {
			unimplemented!()
		}

		pub fn total_asset_issuance(_id: T::AssetId) -> <T as pallet_assets::Config>::Balance {
			unimplemented!()
		}
	}
}
