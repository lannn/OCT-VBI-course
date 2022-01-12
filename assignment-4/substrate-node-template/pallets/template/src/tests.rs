use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
        let claim = Tweet {
            id: 1,
            user_id: 1,
        };

		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            [1, frame_system::Pallet::<Test>::block_number()]
        )
	});
}

#[test]
fn create_claim_failed_when_already_exist() {
	new_test_ext().execute_with(|| {
        let claim = Tweet {
            id: 1,
            user_id: 1,
        };

        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());

		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()), 
            Error::<Test>::ProofAlreadyClaimed
        );
	});
}