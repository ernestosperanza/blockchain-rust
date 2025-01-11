mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();
	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: bob, amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie, amount: 30 }),
			},
		],
	};

	runtime.execute_block(block_1).expect("invalid block");
	assert_eq!(runtime.system.block_number(), 1);

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![support::Extrinsic {
			caller: alice.clone(),
			call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
				claim: "claim",
			}),
		}],
	};

	runtime.execute_block(block_2).expect("invalid block");
	assert_eq!(runtime.system.block_number(), 2);
	println!("{:#?}", runtime);
}
