struct Pet {
    name : String,
    is_cute : bool,
    _type : String,
    age : f32    
}

fn my_pets() {
    let mut fido = Pet { //all fields must be mutable
        name: "Fido",
        _type: "dog",
        is_cute : true,
        age: 0.1  
    };

    fido.age = 0.2; //all fields are equally mutable
    let chase = generate_cute_dog("Chase");
    println!("chase {:?}")
} 

fn generate_cute_dog(name: String) -> Pet{
    Pet {
        name,
        _type: "dog",
        is_cute : true,
        age: 0
    }
}