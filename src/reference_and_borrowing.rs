pub fn moving_reference(){
    println!("--- moving_reference ---");
    // COPY - MOVING OWNERSHIP
    let heap_string = String::from("I'm a moved copy");
    verbose_read_strings_times(heap_string);  // moves the ownerhip to the vars present in the method
   // let second_reading = format!("{}", heap_string); this will not compile: pointer used after its pointee is freed

    // REFERENCE
    let heap_string = String::from("I'm a reference");
    coincise_read_strings_times(&heap_string);
    let second_reading = format!("{}", heap_string);
}

pub fn verbose_read_strings_times(s1: String) -> String {
    println!("{}", s1);
    s1
}

// references are not-owning pointers!
// if verbose_read_strings_times is invoked before >> rustc --explain E0382
pub fn coincise_read_strings_times(s1: &String) {
     println!("{}", s1);
}

pub fn dereferencing(){
    println!("--- dereferencing ---");
    let mut my_heap_int = Box::new(1);
    let my_heap_int_content:i32 = *my_heap_int;
    println!("This is the box content: {}", my_heap_int_content);

    let reference_to_my_int : &i32 = &*my_heap_int;
    let my_int_content:i32 = *reference_to_my_int; 
    println!("This is the box content: {}", my_int_content);

    let reference_to_my_heap_int : &Box<i32> = &my_heap_int;
    let my_heap_int_content:i32 = **reference_to_my_heap_int; //double dereferences
    println!("This is the box content: {}", my_heap_int_content);

    *my_heap_int = 2;
    println!("This is the box content changed by dereference: {}", my_heap_int);
    
    let nested_my_heap_int = Box::new(&my_heap_int);
    let my_int_content:i32 = ***nested_my_heap_int; //one for &, one for first box, one for second box
     println!("This is the box content: {}", my_heap_int);  
}

pub fn explicit_implicit_dereferencing() {
    println!("--- explicit implicit dereferencing ---");
    let my_heap_int = Box::new(-1);
    let explicit_my_heap_int = i32::abs(*my_heap_int);
    let implicit_my_heap_int = my_heap_int.abs(); //most used
    println!("Explicit and implicit dereferencing {} {} ", explicit_my_heap_int, implicit_my_heap_int);

    let reference_to_my_heap_int = &my_heap_int;
    let explicit_my_heap_int = i32::abs(**reference_to_my_heap_int);
    let implicit_my_heap_int = reference_to_my_heap_int.abs(); //it works as multiple levels of pointers dereferencing method
     println!("Explicit and implicit dereferencing {} {} ", explicit_my_heap_int, implicit_my_heap_int);
}

pub fn aliasing(){
    let mut vector = vec![1,2,3];
    let access_2 = &vector[2]; //data aliased
    vector.push(4); //data mutated
    //println("Second element is {}", *access_2); does not compile > push freed access_2 pointer > vector is an another loaction due to reesizing

    let box_aliasing = Box::new(1);
    //to finish
}

pub fn reference_immutable(){
    let x = 0;
    let mut x_ref = &x; //MUTATION x has R, ALIASING *x_ref has NONE
    // *x_ref = 1; not permitted
    println!("Permits aliasing, not mutation {} ", x)
}

pub fn reference_mutable(){
    let mut x = 0;
    let mut x_ref = &mut x; //ALIASING x has NONE, MUTATION *x_ref has RW
    *x_ref = 1;
    println!("Does not permit aliasing, permits mutation {} ", x)
}