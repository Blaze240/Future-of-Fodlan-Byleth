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

mod SpecialHi;
mod SpecialLw;
mod SpecialNStart;
mod SpecialSStart;

pub fn install() {
    SpecialNStart::install();
    SpecialSStart::install();
    SpecialHi::install();
    SpecialLw::install();
}
