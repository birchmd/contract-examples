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
    let pointer = if let Key::Hash(hash) = get_uref("unbonding") {
        ContractPointer::Hash(hash)
    } else {
        revert(66); // exit code is currently arbitrary
    };
    
    // Put the desired unbonding amount here.
    // None means that the complete stake will be unbonded, while
    // Some(x) means an amount x will be unbonded while the rest
    // remains.
    // Note the type of the argument is correct, do not change
    // this when changing the value.
    let arg: Option<U512> = None;
    let purse: Key = main_purse().value().into();
    let _result: () = call_contract(pointer, &arg, &vec![purse]);
}
