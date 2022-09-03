#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use sp_std::vec;
	use sp_std::vec::Vec;
	use bellman_verifier::{ Proof, VerifyingKey};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;


	// 上链的是Vec<u8>类型
	#[pallet::storage]
	#[pallet::getter(fn proof)]
	pub type Pof<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<Something<T>>::put(something);

			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			match <Something<T>>::get() {
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(0)]
		pub fn generate_proof(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult{
			let who = ensure_signed(origin)?;
			<Pof<T>>::insert(who.clone(), proof);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn verifier(origin: OriginFor<T>, ) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// verify_proof(&pvk, &proof, &[]);
			Ok(())
		}
	}
}
