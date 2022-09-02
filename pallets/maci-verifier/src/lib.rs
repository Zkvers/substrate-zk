#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod circuit;
use circuit::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use rand::thread_rng;
	use ff::Field;
	use bls12_381::{Bls12, Scalar};
	use bellman::groth16::{
		create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
	};
    use crate::{MIMC_ROUNDS, mimc, MiMCDemo};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

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
		pub fn verifier(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			// step1
			let mut rng = thread_rng();
			// Generate the MiMC round constants
			let constants = (0..MIMC_ROUNDS).map(|_| Scalar::random(&mut rng)).collect::<Vec<_>>();

			println!("Creating parameters...");

			// Create parameters for our circuit
			let params = {
				let c = MiMCDemo { xl: None, xr: None, constants: &constants };

				generate_random_parameters::<Bls12, _, _>(c, &mut rng).unwrap()
			};

			// step2
			// Prepare the verification key (for proof verification)
			let pvk = prepare_verifying_key(&params.vk);

			println!("Creating proofs...");
			let xl = Scalar::random(&mut rng);
			let xr = Scalar::random(&mut rng);

			let image = mimc(xl, xr, &constants);

			// Create an instance of our circuit (with the
			// witness)
			let c = MiMCDemo { xl: Some(xl), xr: Some(xr), constants: &constants };

			// step3
			// Create a groth16 proof with our parameters.
			let proof = create_random_proof(c, &params, &mut rng).unwrap();

			// step4
			// if verification passed, return ()
			assert!(verify_proof(&pvk, &proof, &[image]).is_ok());
			Ok(())
		}
	}
}
