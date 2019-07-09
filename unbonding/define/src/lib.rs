#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;

extern crate common;
use common::contract_api;
use common::contract_api::pointers::UPointer;
use common::key::Key;
use common::value::uint::U512;

#[no_mangle]
pub extern "C" fn unbonding_ext() {
    let pos_public: UPointer<Key> = contract_api::get_uref("pos").to_u_ptr().unwrap();
    let pos_contract: Key = contract_api::read(pos_public);
    let pos_pointer = pos_contract.to_c_ptr().unwrap();

    let unbond_amount: Option<U512> = contract_api::get_arg(0);
    let _result: () = contract_api::call_contract(pos_pointer, &("unbond", unbond_amount), &vec![]);
}

#[no_mangle]
pub extern "C" fn call() {
    let pos_key = contract_api::get_uref("pos");
    let mint_key = contract_api::get_uref("mint");

    let named_keys: BTreeMap<String, Key> = vec![
        ("pos".into(), pos_key),
        ("mint".into(), mint_key),
    ].into_iter().collect();
    
    let pointer = contract_api::store_function("unbonding_ext", named_keys);
    contract_api::add_uref("unbonding", &pointer.into())
}
