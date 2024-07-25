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

mod SpecialAirHi;
mod SpecialAirLw;
mod SpecialAirNStart;
mod SpecialAirSStart;

pub fn install() {
    SpecialAirNStart::install();
    SpecialAirSStart::install();
    SpecialAirHi::install();
    SpecialAirLw::install();
}
