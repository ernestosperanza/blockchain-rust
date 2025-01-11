use num::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
	type AccountId: Ord + Clone;
	type Nonce: Zero + One + Copy;
	type BlockNumber: Zero + One + AddAssign + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce: T::Nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
		let new_nonce: T::Nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	use crate::system::Config;

	use super::Pallet;

	#[test]
	fn init_system() {
		struct TestConfig;
		impl Config for TestConfig {
			type AccountId = String;
			type BlockNumber = u32;
			type Nonce = u32;
		}
		let alice = "Alice".to_string();
		let bob = "Bob".to_string();
		let mut system = Pallet::<TestConfig>::new();
		assert_eq!(system.block_number(), 0);
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);

		assert_eq!(system.nonce.get(&alice), None);
		system.inc_nonce(alice.clone());
		assert_eq!(system.nonce.get(&alice), Some(&1));
		assert_eq!(system.nonce.get(&bob), None);
	}
}
