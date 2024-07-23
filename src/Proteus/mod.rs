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

mod AppealHi;
mod AppealLw;
mod AppealS;

mod Wait1;

mod Win1;
mod Win2;
mod Win3;

pub fn install() {
    AppealHi::install();
	AppealS::install();
	AppealLw::install();
	Wait1::install();
	Win1::install();
	Win2::install();
	Win3::install();
}