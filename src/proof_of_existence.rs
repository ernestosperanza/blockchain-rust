use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if caller != *owner {
			return Err("this content is owned by someone else");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::Pallet;

	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let claim_one = "claim_one";
		let claim_two = "claim_two";
		let alice = "alice";
		let bob = "bob";
		let mut proof_of_existence = Pallet::<TestConfig>::new();
		// Check the initial state is as you expect.
		assert_eq!(proof_of_existence.get_claim(&claim_one), None);
		// create claim and get
		let _ = proof_of_existence.create_claim(&alice, claim_one);
		assert_eq!(proof_of_existence.get_claim(&claim_one), Some(&alice));
		assert_eq!(proof_of_existence.get_claim(&claim_two), None);
		// Try to claim an existing one
		assert_eq!(
			proof_of_existence.create_claim(&alice, claim_one),
			Err("this content is already claimed")
		);

		// Revoke
		assert_eq!(proof_of_existence.revoke_claim(alice, claim_two), Err("claim does not exist"));
		assert_eq!(
			proof_of_existence.revoke_claim(bob, claim_one),
			Err("this content is owned by someone else")
		);
		assert_eq!(proof_of_existence.revoke_claim(alice, claim_one), Ok(()));
		assert_eq!(proof_of_existence.get_claim(&claim_one), None);
	}
}
