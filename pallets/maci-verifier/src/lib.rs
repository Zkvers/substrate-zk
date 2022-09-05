#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;
pub mod parser;
use types::{ProofStr, VkeyStr};
use parser::{parse_proof, parse_vkey};

#[frame_support::pallet]
pub mod pallet {
	use crate::{ProofStr, VkeyStr};
	use crate::{parse_proof, parse_vkey};
	use bellman_verifier::{prepare_verifying_key, verify_proof};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use bls12_381::Bls12;
	use ff::PrimeField as Fr;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proof_store)]
	pub type Pof<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ProofStr, OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn vkey_store)]
	pub type Vkey<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, VkeyStr, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProofStored(ProofStr, T::AccountId),
		VerificationKeyStore(VkeyStr, T::AccountId),
		VerificationPassed(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoProof,
		NoVerificationKey,
		VerificationFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn generate_proof_vkey(
			origin: OriginFor<T>,
			proof_a: Vec<u8>,
			proof_b: Vec<u8>,
			proof_c: Vec<u8>,
			vk_alpha1: Vec<u8>,
			vk_beta_2: Vec<u8>,
			vk_gamma_2: Vec<u8>,
			vk_delta_2: Vec<u8>,
			vk_ic: Vec<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let proof = ProofStr { pi_a: proof_a, pi_b: proof_b, pi_c: proof_c };
			let vkey = VkeyStr {
				alpha_1: vk_alpha1, 
				beta_2: vk_beta_2,
				gamma_2: vk_gamma_2,
				delta_2: vk_delta_2,
				ic: vk_ic,
			};
			<Pof<T>>::insert(who.clone(), &proof);
			<Vkey<T>>::insert(who.clone(),&vkey);
			Self::deposit_event(Event::<T>::ProofStored(proof, who.clone()));
			Self::deposit_event(Event::<T>::VerificationKeyStore(vkey, who));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn verifier(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match <Pof<T>>::get(&who) {
				None => return Err(Error::<T>::NoProof.into()),
				Some(pof) => {
					log::info!("{:?}", pof.pi_a);
					let proof = parse_proof::<Bls12>(pof.clone());
					log::info!("{:?}", proof.a);

					match <Vkey<T>>::get(&who) {
						None => return Err(Error::<T>::NoVerificationKey.into()),
						Some(vkeystr) => {
							log::info!("{:?}",vkeystr.alpha_1);
							let vkey = parse_vkey::<Bls12>(vkeystr);
							log::info!("{:?}",vkey.clone().alpha_g1);

							let pvk =  prepare_verifying_key(&vkey);

							match verify_proof(&pvk, &proof, &[Fr::from_str_vartime("33").unwrap()]) {
								Ok(()) => Self::deposit_event(Event::<T>::VerificationPassed(who)),
								Err(e) => { 
									log::info!("{:?}", e);
									()
								}
							}
						}
					}
				},
			}

			Ok(())
		}
	}
}
