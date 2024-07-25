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

mod Aerials;
mod Grounded_Moves;
mod Other;
//mod SpecialAir;
//mod Special;

pub fn install() {
    Aerials::install();
    Grounded_Moves::install();
    Other::install();
    
}
