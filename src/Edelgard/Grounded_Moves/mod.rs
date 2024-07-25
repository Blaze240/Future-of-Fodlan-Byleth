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

mod AttackDash;

mod Attack11;

mod AttackS3;
mod AttackHi3;
mod AttackLw3;

mod AttackS4;
mod AttackHi4;
mod AttackLw4;

pub fn install() {
	Attack11::install();
	AttackDash::install();

	AttackS3::install();
	AttackHi3::install();
	AttackLw3::install();

    AttackS4::install();
	AttackHi4::install();
	AttackLw4::install();
}