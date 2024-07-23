#![feature(
    concat_idents,
    proc_macro_hygiene
)]
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

mod Dark_Knight_Switch;
mod Paladin_Switch;
mod Proteus;

#[skyline::main(name = "soulshift_marth")]
pub fn main() {
    Dark_Knight_Switch::install();
	Paladin_Switch::install();
	Proteus::install();
}