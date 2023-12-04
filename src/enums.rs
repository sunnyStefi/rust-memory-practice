enum Toys {
    Puzzle(String),//field is function
    Cars(u8,u8,u8) //stack
}

impl Toys{
    fn pick(&self) {
        match self {
            Toys::Puzzle(description) => {
                println!("Puzzle: {}", description);
            }
            Toys::Cars(speed, year, model) => {
                println!("Cars - Speed: {}, Year: {}, Model: {}", speed, year, model);
            }
        }
    }
}

pub fn play_with_activity(){
    let puzzle_toys = Toys::Puzzle(String::from("Paw Patrol"));
    let cars_toys = Toys::Cars(1,4,6); 
    puzzle_toys.pick();
    cars_toys.pick();
}