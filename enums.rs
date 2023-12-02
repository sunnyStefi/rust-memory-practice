enum Toys {
    Puzzle(String),//field is function
    Cars(u8,u8,u8) //stack
}


pub fn play_with_activity(){
    let puzzle_toys = Toys::Puzzle(String::from("Paw Patrol"));
    let cars_toys = Toys:Cars(1,4,6,3); 
}