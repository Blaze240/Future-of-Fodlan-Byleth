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

mod AttackDash;
mod Wait1;

mod SpecialHi;
mod SpecialLw;
mod SpecialLwHit;
mod SpecialNStart;
mod SpecialS1;

mod SpecialAirHi;
mod SpecialAirLw;
mod SpecialAirLwHit;
mod SpecialAirNStart;
mod SpecialAirS1;

mod Win1;
mod Win2;
mod Win3;

pub fn install() {
    AppealHi::install();
    AppealS::install();
    AppealLw::install();
    Wait1::install();
    AttackDash::install();

    SpecialNStart::install();
    SpecialS1::install();
    SpecialHi::install();
    SpecialLw::install();
    SpecialLwHit::install();

    SpecialAirNStart::install();
    SpecialAirS1::install();
    SpecialAirHi::install();
    SpecialAirLw::install();
    SpecialAirLwHit::install();

    Win1::install();
    Win2::install();
    Win3::install();
}
