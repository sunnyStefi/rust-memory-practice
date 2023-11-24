use std::rc::Rc;
/* 6 most common Ownership Errors 
*/
// 1. returning a reference to a function's local variable is not possible
// because the local variable is dropped when the function finishes its execution
// so the referenced data is not longer accessible

pub fn i_hate_bugs(){
    // let bug0 = returning_reference_is_not_possible();
    let bug1 = returning_reference_is_not_possible_1();
    let bug2 = returning_reference_is_not_possible_2();
    let bug3 = returning_reference_is_not_possible_3();
    let mut bug4 = String::new();
    returning_reference_is_not_possible_4(&mut bug4);
    println!("{bug1} {bug2} {bug3} {:?}", bug4);
}
// fn returning_reference_is_not_possible()->&String{
//     let bug = String::from("cucaracha");
//     &bug //the underline string will be deallocated when this function finishes
// }

fn returning_reference_is_not_possible_1()-> String{
    let bug = String::from("ladybug");
    bug
}
fn returning_reference_is_not_possible_2()-> &'static str{
    "spider"
}
//referece counting: cloning the pointer, not its content > runtime checks
fn returning_reference_is_not_possible_3()-> Rc<String>{
     let bug = Rc::new(String::from("centipede"));
     Rc::clone(&bug) 
}
//refactor the function signature
fn returning_reference_is_not_possible_4(bug: &mut String){
     bug.replace_range(..,"cockroach")
} 

// 2.1 passing immutable &ref as param and want to work with its content
// ISSUE: similar to 3.1

pub fn my_supplements(){
    // solution 1: make mysupplements mut but that could be different from the user's needs
    let my_supplements = vec![String::from("vitamin D"), String::from("vitamin C")];
    let my_first_supplement = &my_supplements[0];
    // add_magnesium_with_issue(&my_supplements); // starting point
    // add_magnesium_with_ownership(my_supplements); //solution 2 joint to solution 1
    let all = add_magnesium(&my_supplements);
    println!("{all}");
}

//starting point
// fn add_magnesium_with_issue(supplements:& Vec<String>) -> String{
//     supplements.push(String::from("magnesium"));//ISSUE
//     let all = supplements.join(" ");
//     all
// }
//solution 2: removing & >> very rare! because it makes my_supplements unusable in the last line and supplmnts
// fn add_magnesium_with_ownership(mut supplements: Vec<String>) -> String{
//     supplements.push(String::from("magnesium"));//ISSUE
//     let all = supplements.join(" ");
//     all
// }
//solution 3: clone the reference and work with it
fn add_magnesium(supplements: &Vec<String>) -> String{
    let mut editable_supplments_for_this_function = supplements.clone();
    editable_supplments_for_this_function.push(String::from("magnesium"));//ISSUE
    let all = editable_supplments_for_this_function.join(" ");
    all
}


// 3.1 use reference to heap that gets deallocated by an alias
// ISSUE: destination.push(city.clone()) tries to (W) on destination 
// while max_small_city keeps the reference to destination and (R)
// SOLUTION: shorten max_small_city lifetime
pub fn city_names(){
    let my_big_cities = vec![String::from("cardanoalcampo"),String::from("alcaladehenares"),String::from("madrid")];
    let mut my_not_so_small_cities = vec![String::from("denhaag"),String::from("gandia"), String::from("valencia")];
    add_big_cities(&mut my_not_so_small_cities, &my_big_cities);
    println!("Only bigger cities are added to my cities!");
    println!("{:?}", my_not_so_small_cities);
}

fn add_big_cities(destination: &mut Vec<String>, source: &[String]){
    //solution 1: clone max_small_city
    let max_small_city = destination.iter().max_by_key(|s| s.len()).unwrap().clone(); //max_big_city is a ALIAS reference to the longest string: it removes (WO) on destination until it's out of scope
    
    //solution 2 BEST: we need only the length of the string, not all its reference
    //let max_small_city = destination.iter().max_by_key(|s| s.len()).unwrap().len(); > destination get its permissions back after this line
    //and then use it in the if below 

    //solution 3: copy the source result to another temp vector and add this vector to destination

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
    let nunafen_content = *nunafen_ref + 1; //copying i32. It does implement Copy.
    print!("{}", nunafen_content); //OK

    let plants = vec![String::from(" Nunafen")]; // String does own heap data -> it can be copied with move of ownership
    let nunafen_ref = &plants[0];
    //let nunafen_content = *nunafen_ref; //copying String, but it does NOT implement Copy.
    println!("{}",nunafen_ref);
}

// 4.2 modify array content
pub fn changing_plants(){
    //0 change plants to mut and modify as 4.1
    let mut plants = vec![String::from("Nunafen")];
    let mut first_plant = &mut plants[0];
    *first_plant = String::from("Cactus");
    println!("{}",plants[0]);

    //1 or remove the current element and push a new char element
    let mut old_plants = plants.remove(0); //move out the string from the vector
    old_plants.push('!');
    println!("{}",old_plants);

    //2 if we want to read plants[0], just use &
    //3 if we want to use it with ownership just use clone
}

//  5.1 mutate different tuple fields
//  >> with &my_tuple.0, the permissions reset are applied only to the 1st element
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

    //cant write to my_tuple anymore because it has been borrowed and it has lost OW permissions
    // my_tuple.1.push_str("cars!");
}

fn idontlike_blue_cars_get_first_part(tuple: &(String, String)) -> &String{
    &tuple.0
}
//  6.1 mutate different array elements - incorrect
//  &array[index] borrows permission for all the other indexes
pub fn add_only_white_horses(){
    let mut horses = [1,3]; //horses (RWO), horses[0] (RW)
    let add_white_horse = &mut horses[0]; //horses (-), add_white_horse(RO), *add_white_horse (RW) > borrowed for all indexes
    *add_white_horse += 1;

    let add_black_horse = &mut horses[1]; // horses (-), add_black_horse(RO), *add_black_horse(RW) > borrowed for all indexes
    //*add_white_horse +=1;// cannot borrow `horses[_]` more than once at a time
}
//  6.2 mutate different array elements
//  slit_at_mut uses unsafe to bypass the borrow checker
pub fn add_both_horses(){
    let mut horses = [1,3]; //horses (RWO), horses[0] (RW)
    let (first_part, second_part) = horses.split_at_mut(1);
    let white_horses = &mut first_part[0];
    let black_horses = &mut second_part[0];
    *white_horses +=1;
    println!("{white_horses} {black_horses}")

}