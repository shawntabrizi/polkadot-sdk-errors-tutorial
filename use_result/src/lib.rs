type DispatchResult = Result<(), &'static str>;

pub mod pallet {
	use super::*;

	pub trait Config {
		type AccountId;
		type Balance;
	}

	pub struct Pallet<T>(core::marker::PhantomData<T>);

	impl<T: Config> Pallet<T> {
		pub fn transfer(
			_from: T::AccountId,
			_to: T::AccountId,
			_amount: T::Balance,
		) -> DispatchResult {
			unimplemented!()
		}

		pub fn buy_item(
			who: T::AccountId,
			from: T::AccountId,
			price: T::Balance,
		) -> DispatchResult {
			Self::transfer(who, from, price);
			Ok(())
		}
	}
}
