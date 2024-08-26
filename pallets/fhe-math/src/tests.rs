use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);
        // Dispatch a signed extrinsic.
        assert_ok!(TemplateModule::encrypt_numbers(
            RuntimeOrigin::signed(1),
            10,
            20
        ));
        assert_ok!(TemplateModule::decrypt_result(
            RuntimeOrigin::signed(1),
            0,
            "add".to_string()
        ));
        // TemplateModule::decrypt_result(RuntimeOrigin::signed(1), 0)
    });
}
