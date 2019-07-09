#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

extern crate common;
use common::contract_api::pointers::ContractPointer;
use common::contract_api::{call_contract, get_uref, main_purse, revert};
use common::key::Key;
use common::value::U512;

#[no_mangle]
pub extern "C" fn call() {
    let pointer = if let Key::Hash(hash) = get_uref("bonding") {
        ContractPointer::Hash(hash)
    } else {
        revert(66); // exit code is currently arbitrary
    };
    let purse: Key = main_purse().value().into();
    let arg = U512::from(1000); // replace with your desired bonding amount
    let _result: () = call_contract(pointer, &arg, &vec![purse]);
}
