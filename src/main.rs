mod reference_and_borrowing;
mod fixing_ownership_errors;

fn main() {
    fixing_ownership_errors_cases();
}


fn reference_and_borrowing_cases(){
    reference_and_borrowing::moving_reference();
    reference_and_borrowing::dereferencing();
    reference_and_borrowing::explicit_implicit_dereferencing();
    reference_and_borrowing::aliasing();
    reference_and_borrowing::reference_mutable();
    reference_and_borrowing::reference_immutable();
}
//chapter
fn fixing_ownership_errors_cases(){
    // 3.1 use reference to heap that gets deallocated by an alias
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

