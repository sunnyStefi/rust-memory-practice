pub fn vectors_() {
    let victor = vec![1,2,3,4];
    // let lets_panic = victor[4]; //panic: program will crash
    let fourth_option = victor.get(4);

    match fourth_option {
        Some(fourth_option) => println!("{fourth_option}"),
        None => println!("There is no fourth_option"),
    }
}