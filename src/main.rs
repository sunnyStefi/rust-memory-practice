mod reference_and_borrowing;
mod fixing_ownership_errors;
mod structs;
mod enums;
mod matching;
mod modules;
mod common_collections;
mod error;
mod generics;
mod ownership_inventory;
mod test;

fn main() {
    fixing_ownership_errors_cases();
    strucs_enum_matching();
    modules::modules_();
    common_collections();
    errors_();
    generics_();
    ownership_inventory_();
}

fn ownership_inventory_(){
    ownership_inventory::perform_find_nth_element();
}

fn generics_ () {
    generics::print_largest_number();
    generics::print_largest_generic();
    generics::use_generic_struct();
    generics::use_generic_struct_method();
    generics::use_trait();
    generics::use_lifetimes();
}

fn errors_(){
    error::lets_panic();
    error::lets_recover();
}

fn reference_and_borrowing_cases(){
    reference_and_borrowing::moving_reference();
    reference_and_borrowing::dereferencing();
    reference_and_borrowing::explicit_implicit_dereferencing();
    reference_and_borrowing::aliasing();
    reference_and_borrowing::reference_mutable();
    reference_and_borrowing::reference_immutable();
}

fn strucs_enum_matching(){
    structs::my_pets();
    enums::play_with_activity();
    matching::corresponding_cents();
}

fn common_collections(){
    common_collections::vectors_();
    common_collections::strings_();
    common_collections::hash_maps_();
}

fn fixing_ownership_errors_cases(){
    // 1 returning a reference to a function's local variable is not possible
    fixing_ownership_errors::i_hate_bugs();
    // 2.1 passing immutable &ref as param and want to edit its content
    // 2.2 using a reference while its data has been previously dropped by a function
    fixing_ownership_errors::my_supplements();
    // 3.1 using a reference while its data has been previously taken by an alias (akin to 2.2)
    fixing_ownership_errors::city_names();
    // 4.1 modify array content - incorrect
    fixing_ownership_errors::my_plants();
    // 4.2 modify array content
    fixing_ownership_errors::changing_plants();
    // 5.1 mutate different tuple fields
    fixing_ownership_errors::ilike_red_cars();
    // 5.2 mutate different tuple fields - incorrect
    fixing_ownership_errors::idontlike_blue_cars();
    // 6.1 mutate different array elements - incorrect
    fixing_ownership_errors::add_only_white_horses();
    // 6.2 mutate different array elements
    fixing_ownership_errors::add_both_horses();
}

