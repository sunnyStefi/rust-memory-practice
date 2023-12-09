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
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"),10);  //Strings are ownd by th hashmap once inserted: cant reference thm
    scores.insert(String::from("Blue"),3);

    let requested_label= String::from("Blue");
    let non_existing_label = String::from("Yellow");

    let value = scores.get(&requested_label).copied().unwrap_or(0); //copied gets i32 instead &i32
    let zero = scores.get(&non_existing_label); //get gives Option<T>

    println!("Hash map values {value} {:?}", zero);

    for (key, value) in &scores {
        println!("{key} scores is {value}");
    }

    scores.insert(String::from("Blue"),19); //this will overwrite 3
    scores.entry(String::from("Blue")).or_insert(20); //insert only if Blue does not exist

    println!("All scores {:?}", scores);

    let main_string = "i like coffee";
    let mut new_hashmap = HashMap::new();
    
    for word in main_string.split_whitespace(){
        let counter = new_hashmap.entry(word).or_insert(0);  //or insert gives a &mut for the value!
        *counter +=1;
    }

    println!("new hashmap {:?}",new_hashmap);

    let mut new_hashmap : HashMap<char, Vec<usize>>= HashMap::new();
    for (index, charact) in "i like coffee".chars().enumerate(){
        new_hashmap.entry(charact).or_insert(Vec::new()).push(index);
    }

    println!("new hashmap {:?}",new_hashmap);
}