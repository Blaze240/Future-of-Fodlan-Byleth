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

mod AttackAirN;
mod AttackAirF;
mod AttackAirB;
mod AttackAirHi;
mod AttackAirLw;

pub fn install() {
    AttackAirN::install();
	AttackAirF::install();
	AttackAirB::install();
	AttackAirHi::install();
	AttackAirLw::install();
}