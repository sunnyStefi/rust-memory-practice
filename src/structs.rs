#[derive(Debug)]
struct Pet {
    name : String,
    is_cute : bool,
    _type : String,
    age : f32,
    constant: u32
}
//Each struct is allowed to have multiple impl blocks
impl Pet {
    fn human_age(&self) -> f32 {
        self.age * (self.constant as f32)
    }
    //method with more params
    fn set_age(&mut self, new_age:f32) {
        self.age = new_age
    }

    fn create_pet_with_older_age(self, other: Pet) -> Pet { //rare: if we move a intance, this fn will get ownership
        Pet {
             name : String::from("Rocky"),
            is_cute : true,
            _type : String::from("dog"),
            age : self.age.max(other.age),
            constant: 7
        }
    }

    fn set_to_max_age(&mut self, other:Pet) {
        //here self has (RO) permissions ->see set_age
        *self = self.create_pet_with_older_age(other); // wrong: moving ownership to create_pet..
    }

    fn is_younger(&self, other:&Pet) -> bool {
        self.age < other.age
    }
    //associated function
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
    println!("---------- Structs ----------");
    //Rust does not have a keyword for constructor functions.
    //The idiomatic way to define a constructor function is to make an associated function called new,
    let mut fido = Pet { //all fields must be mutable
        name: String::from("Fido"),
        _type: String::from("dog"),
        is_cute : true,
        age: 0.1,
        constant: 7
    };

    fido.age = 0.2; //all fields are equally mutable
    fido.set_age(0.3); //equivalent to Pet::set_age(&mut fido, num)
    Pet::set_age(&mut fido, 4.0); //equivalent to fido.set_age(num)
    println!("fido new age {}", fido.age);
    let name = String::from("Chase");
    let chase = generate_cute_dog(name);
    println!("chase {:?}", chase);
    let fido_human_age = fido.human_age();
    let fido_human_age = Pet::human_age(&fido);
    println!("fido human age {}", fido_human_age);
    println!("chase is younger than fido? {}", chase.is_younger(&chase));
    let old_dog = Pet::ten_year_old_dog(String::from("Old dog"));
    println!("old dog {:?}", old_dog);

    let skye = &mut Box::new(Pet{    
        name: String::from("Skye"),
        _type: String::from("dog"),
        is_cute : true,
        age: 1.0,
        constant: 7});
    
    //syntactic sugar: 
    let skye_human_age = skye.human_age();
    let skye_human_age = Pet::human_age(&skye); // equivalent to Pet::human_age(&**skye); Rust will add two dereferences (once for the mutable reference, once for the box) and then one immutable borrow
    println!("skye human age {}", skye_human_age);

    //Methods and Ownership
    println!("-- Methods and Ownership");
    println!("fido and chase age {} {}", fido.age, chase.age);
    let old_rocky = fido.create_pet_with_older_age(chase); 
    println!("new old pet rocky {:?}", old_rocky);
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