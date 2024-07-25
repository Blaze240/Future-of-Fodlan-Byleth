#![feature(concat_idents, proc_macro_hygiene)]
#![allow(
    unused_imports,
    unused_macros,
    unused_variables,
    unused_assignments,
    unused_unsafe,
    non_upper_case_globals,
    non_snake_case,
    clippy::borrow_interior_mutable_const
)]
static mut TIMESKIP_SWITCH: bool = false;
static mut PROMOTION_SWITCH: bool = false;
static mut HOPES_SWITCH: bool = false;
static mut DEFAULT_SWITCH: bool = false;

mod Edelgard;

#[skyline::main(name = "future_of_fodlan_byleth")]
pub fn main() {
	Edelgard::install();
}
