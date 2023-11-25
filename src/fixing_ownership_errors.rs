use std::rc::Rc;
// 6 most common Ownership Errors 

// 1. returning a reference to a function's local variable is not possible
// because the local variable is dropped when the function finishes its execution
// so the referenced data is not longer accessible
// 4 solutions are available
pub fn i_hate_bugs(){
    // let bug0 = returning_reference_is_not_possible(); // not a solution
    let bug1 = returning_reference_is_not_possible_solution_1();
    let bug2 = returning_reference_is_not_possible_solution_2();
    let bug3 = returning_reference_is_not_possible_solution_3();
    let mut bug4 = String::new();
    returning_reference_is_not_possible_solution_4(&mut bug4);
    println!("{bug1} {bug2} {bug3} {:?}", bug4);
}

// not a solution: you cannot return a reference to a function' local variable
// fn returning_reference_is_not_possible()->&String{
//     let bug = String::from("cucaracha");
//     &bug //the data "cucaracha" will be deallocated when this function finishes
// }

// solution 1: returning the whole variable, not only its reference
// and move its ownership out of the function (content of bug belongs to bug1)
fn returning_reference_is_not_possible_solution_1()-> String{
    let bug = String::from("ladybug");
    bug
}
// solution 2: returning a stack literal
fn returning_reference_is_not_possible_solution_2()-> &'static str{
    "spider"
}
// solution 3: cloning the pointer itself with Rc.
// The check about if the pointing to the data is dropped will be done at runtime
fn returning_reference_is_not_possible_solution_3()-> Rc<String>{
     let bug = Rc::new(String::from("centipede"));
     Rc::clone(&bug) 
}
// solution 4: refactor the function signature with a mutable reference
fn returning_reference_is_not_possible_solution_4(bug: &mut String){
     bug.replace_range(..,"cockroach")
} 

// 2.1 passing immutable &ref as param and want to edit its content
// 2.2 using a reference while its data has been previously dropped (after a function)
pub fn my_supplements(){
    let my_supplements = vec![String::from("vitamin D"), String::from("vitamin C")];
    let my_first_supplement = &my_supplements[0];
    // changing_read_only_ref_is_not_possible(&my_supplements); // not a solution
    // changing_read_only_ref_is_not_possible_solution_2(my_supplements); //solution 2
    let all = changing_read_only_ref_is_not_possible_solution_3(&my_supplements);
    println!("{all}");
}

// not a solution:
// fn changing_read_only_ref_is_not_possible(supplements:& Vec<String>) -> String{
//     supplements.push(String::from("magnesium"));//ISSUE
//     let all = supplements.join(" ");
//     all
// }

// solution 1: gelato solution make my_supplements &mut
// but that could be different from the user's needs

// solution 2: removing reference, BUT is very rare
// because it makes my_supplements unusable in the main program (last line).
// it generates issue 2.2
// fn changing_read_only_ref_is_not_possible_solution_2(mut supplements: Vec<String>) -> String{
//     supplements.push(String::from("magnesium"));//ISSUE
//     let all = supplements.join(" ");
//     all
// }

//solution 3: clone the reference and work with it
fn changing_read_only_ref_is_not_possible_solution_3(supplements: &Vec<String>) -> String{
    let mut editable_supplments_for_this_function = supplements.clone();
    editable_supplments_for_this_function.push(String::from("magnesium"));//ISSUE
    let all = editable_supplments_for_this_function.join(" ");
    all
}

// 3.1 using a reference while its data has been previously take by an alias (akin to 2.2)
// destination.push(city.clone()) tries to (W) on destination 
// while &max_small_city keeps the reference to destination and gives it (R) permissions
// SOLUTION: shorten max_small_city lifetime
pub fn city_names(){
    let my_big_cities = vec![String::from("cardanoalcampo"),String::from("alcaladehenares"),String::from("madrid")];
    let mut my_small_cities = vec![String::from("denhaag"),String::from("gandia"), String::from("valencia")];
    add_big_cities(&mut my_small_cities, &my_big_cities);
    println!("Only bigger cities than 'valencia' are added to my cities!");
    println!("{:?}", my_small_cities);
}

fn add_big_cities(destination: &mut Vec<String>, source: &[String]){
    // solution 1: clone max_small_city
    let max_small_city : &String = destination.iter().max_by_key(|s| s.len()).unwrap().clone(); //max_big_city is a ALIAS/reference to the longest string: it removes (WO) on destination until it's out of scope
    
    // solution 2 BEST: we need only the length of the string, not all its reference
    // let max_small_city = destination.iter().max_by_key(|s| s.len()).unwrap().len(); > destination get its permissions back after this line
    // and then use it in the if below 

    // solution 3: copy the computation result on source to another temp vector and extend destination with this vector
    // let temp: Vec<String> = source.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    // destination.extend(temp);

    for city in source {
        if city.len() > max_small_city.len(){
            destination.push(city.clone()); //ISSUE withouth clone(): push needs (W) but we have to wait until max_small_city is dropped: we need to shorten its lifetime
        }
    }
}

//  4.1 modify array content - incorrect
pub fn my_plants(){
    let plants = vec![1,1,1]; //i32 does not own heap data: it can be copied without move of ownership
    let nunafen_ref = &plants[0];
    let nunafen_content = *nunafen_ref + 1; //moving-copying i32. It does implement Copy. note that nunafen_ref is not mut, but what it points to is
    print!("{}", nunafen_content); //OK

    let plants = vec![String::from(" Nunafen")]; // String does own heap data -> it can be copied with move of ownership
    let nunafen_ref = &plants[0];
    //let nunafen_content = *nunafen_ref; //moving-copying String, but it does NOT implement Copy.
    println!("{}",nunafen_ref);
}

//  4.2 modify array content
pub fn changing_plants(){
    //solution 1: change plants to mut and modify as 4.1
    let mut plants = vec![String::from("Nunafen")];
    let mut first_plant = &mut plants[0];
    *first_plant = String::from("Cactus");
    println!("{}",plants[0]);

    //solution 2; remove the current element and push a new char element
    let mut old_plants = plants.remove(0); //move out the string from the vector
    old_plants.push('!');
    println!("{}",old_plants);

    //2 if we want to read plants[0], just use &
    //3 if we want to use it with ownership just use clone
}

//  5. when borrowed as a whole tuple, all its element looses (WO) permission
//  instead, passing only &my_tuple.0, the permissions reset are applied only to the part that's borrowed (1st element)
pub fn ilike_red_cars(){
    let mut my_tuple = (String::from("I like "), String::from("red "));
    let first_part = &my_tuple.0; // my_tuple.0 (R) my_tuple.1 (RWO)
    print!("{} {} ", my_tuple.0, &my_tuple.0); //their content is the same

    my_tuple.1.push_str("cars!");
    println!("{}", my_tuple.1);
}
//  5.2 mutate different tuple fields - incorrect
//  >> with &my_tuple, the permissions reset are applied to the whole tuple
//  SOLUTION: defer borrow checking to runtime with cells
pub fn idontlike_blue_cars(){
    let mut my_tuple = (String::from("I don't like "), String::from("blue "));
    let first_part = idontlike_blue_cars_get_first_part(&my_tuple); //my_tuple.0 (R), my_tuple.1 (R)
    //cant write to my_tuple anymore because the whole tuple has been borrowed and it has lost (WO) permissions
    // my_tuple.1.push_str("cars!");
}

fn idontlike_blue_cars_get_first_part(tuple: &(String, String)) -> &String{
    &tuple.0
}


//  6.1 opposite of point 5.1
//  when borrowing one element of the array, the whole array has (WO) removed
//  mutate different array elements - incorrect
pub fn add_only_white_horses(){
    let mut horses = [1,3]; //horses (RWO), horses[0] (RW)
    let add_white_horse = &mut horses[0]; //horses (-), add_white_horse(RO), *add_white_horse (RW) > borrowed for all indexes
    *add_white_horse += 1;
    //add_white_horse will loose horse ownership
    let add_black_horse = &mut horses[1]; // horses (-), add_black_horse(RO), *add_black_horse(RW) > borrowed for all indexes
    //*add_white_horse +=1;// cannot borrow `horses[_]` more than once at a time
}
//  6.2 mutate different array elements
//  split_at_mut uses unsafe to bypass the borrow checker
pub fn add_both_horses(){
    let mut horses = [1,3]; //horses (RWO), horses[0] (RW)
    let (first_part, second_part) = horses.split_at_mut(1);
    let white_horses = &mut first_part[0];
    let black_horses = &mut second_part[0];
    *white_horses +=1;
    println!("{white_horses} {black_horses}")

}