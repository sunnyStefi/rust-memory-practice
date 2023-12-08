use std::collections::HashMap;
pub fn vectors_() {
    let mut victor = vec![1,2,3,4];
    // let lets_panic = victor[4]; //panic: program will crash
    let fourth_option = victor.get(4);

    match fourth_option {
        Some(fourth_option) => println!("{fourth_option}"),
        None => println!("There is no fourth_option"),
    }

    //QUIZ - Question 2: push will move the variable: it will not be usable afterwards
    let five = 5;
    victor.push(five); //five is moved by push, so it's not usable
    // println!(five); not usable -> not ownership safe

    //QUIZ - Question 3: non-copyable types cannot be moved out of a vector by indexing
    let mut ugo = vec![String::from("Ugo")];
    // let mut first_element = ugo[0]; //cannot move out of index of `Vec<String>` 4.3 
    let first_element = ugo.remove(0); //#3
}

pub fn strings_(){
    let s1 = String::from("con");
    let s2 = String::from("cat");
    let s3 = s1 + &s2;
    println!("{}",s3);
}

pub fn hash_maps_(){
    let scores = HashMap::new();
    scores.insert(String::from("Red"),10);
    scores.insert(String::from("Blue"),3);
}