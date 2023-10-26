use codec::{Decode, Encode};
use frame_support::pallet_prelude::RuntimeDebug;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, Default, PartialEq, RuntimeDebug, scale_info::TypeInfo)]
pub struct ProofStr {
	pub pi_a: Vec<u8>,
	pub pi_b: Vec<u8>,
	pub pi_c: Vec<u8>,
}

#[derive(Clone, Encode, Decode, Default, PartialEq, RuntimeDebug, scale_info::TypeInfo)]
pub struct VkeyStr {
	pub alpha_1: Vec<u8>,
	pub beta_2: Vec<u8>,
	pub gamma_2: Vec<u8>,
	pub delta_2: Vec<u8>,
	pub ic0: Vec<u8>,
	pub ic1: Vec<u8>,
}
