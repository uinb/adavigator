use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use sp_core::{H160, H256, U256};
use std::str::FromStr;

#[test]
fn test_create() {
	let alice = MyAccountId::from_str("0x1234500000000000000000000000000000000000").unwrap();
	let origin = RuntimeOrigin::signed(alice);
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RuntimeOrigin::root(),
			H160::default().into(),
			1000_000_000_000_000_000_000_000_000,
			100_000
		));

		assert_ok!(EVMPermission::create(
			RuntimeOrigin::root(),
			H160::default(),
			b"0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea26469706673582212209a159a4f3847890f10bfb87871a61eba91c5dbf5ee3cf6398207e292eee22a1664736f6c63430008070033".to_vec(),
			U256::default(),
			21000000u64,
			U256::from(9_000_000_000_000u64),
			None,
			Some(U256::from(0)),
			Vec::new(),
		));

		assert_noop!(EVMPermission::create(
			origin,
			H160::default(),
			b"0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea26469706673582212209a159a4f3847890f10bfb87871a61eba91c5dbf5ee3cf6398207e292eee22a1664736f6c63430008070033".to_vec(),
			U256::default(),
			21000000u64,
			U256::from(9_000_000_000_000u64),
			None,
			Some(U256::from(0)),
			Vec::new(),
		),
		sp_runtime::traits::BadOrigin,
	);

	});
}

#[test]
fn test_create2() {
	let alice = MyAccountId::from_str("0x1234500000000000000000000000000000000000").unwrap();
	let origin = RuntimeOrigin::signed(alice);
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RuntimeOrigin::root(),
			H160::default().into(),
			1000_000_000_000_000_000_000_000_000,
			100_000
		));

		assert_ok!(EVMPermission::create2(
			RuntimeOrigin::root(),
			H160::default(),
			b"0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea26469706673582212209a159a4f3847890f10bfb87871a61eba91c5dbf5ee3cf6398207e292eee22a1664736f6c63430008070033".to_vec(),
			H256::random(),
			U256::default(),
			21000000u64,
			U256::from(9_000_000_000_000u64),
			None,
			Some(U256::from(0)),
			Vec::new(),
		));

		assert_noop!(EVMPermission::create2(
			origin,
			H160::default(),
			b"0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea26469706673582212209a159a4f3847890f10bfb87871a61eba91c5dbf5ee3cf6398207e292eee22a1664736f6c63430008070033".to_vec(),
			H256::random(),
			U256::default(),
			21000000u64,
			U256::from(9_000_000_000_000u64),
			None,
			Some(U256::from(0)),
			Vec::new(),
		),
		sp_runtime::traits::BadOrigin,
	);
	});
}

#[test]
fn test_call() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RuntimeOrigin::root(),
			H160::default().into(),
			1000_000_000_000_000_000_000_000_000,
			100_000
		));

		assert_ok!(EVMPermission::call(
			RuntimeOrigin::root(),
			H160::default(),
			H160::from_str("1000000000000000000000000000000000000001").unwrap(),
			Vec::new(),
			U256::default(),
			1000000,
			U256::from(1_000_000_000_000u64),
			None,
			None,
			Vec::new(),
		));

		assert_ok!(EVMPermission::call(
			RuntimeOrigin::root(),
			H160::default(),
			H160::from_str("1000000000000000000000000000000000000002").unwrap(),
			Vec::new(),
			U256::default(),
			1000000,
			U256::from(1_000_000_000_000u64),
			None,
			None,
			Vec::new(),
		));
	});
}
