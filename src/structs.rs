#[derive(Debug)]
struct Pet {
    name : String,
    is_cute : bool,
    _type : String,
    age : f32,
    constant: u32
}

impl Pet {
    fn human_age(&self) -> f32 {
        self.age * (self.constant as f32)
    }

    fn is_younger(&self, other:&Pet) -> bool {
        self.age < other.age
    }

    fn ten_year_old_dog(name: String) -> Self {
        Pet {
            name,
            is_cute : false,
            _type : String::from("dog"),
            age : 10.0,
            constant: 7
        }
    }
}

pub fn my_pets() {
    let mut fido = Pet { //all fields must be mutable
        name: String::from("Fido"),
        _type: String::from("dog"),
        is_cute : true,
        age: 0.1,
        constant: 7
    };

    fido.age = 0.2; //all fields are equally mutable
    let name = String::from("Chase");
    let chase = generate_cute_dog(name);
    println!("chase {:?}", chase);
    println!("fido human age {}", fido.human_age());
    println!("chase is younger than fido? {}", chase.is_younger(&chase));
    let old_dog = Pet::ten_year_old_dog(String::from("Old dog"));
    println!("old dog {:?}", old_dog);
} 

fn generate_cute_dog(name: String) -> Pet{
    Pet {
        name,
        _type: dbg!("dog".to_string()),
        is_cute : true,
        age: 0.0,
        constant: 7
    }
}