//1

//2-1
fn find_nth_element<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}
#[derive(Debug, Clone)] //todo
 struct Person {
        name: String,
        age: u32,
    }

impl Ord for Person {
    fn cmp (&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Person {}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool { //self is a Module, Self is a Type
        (self.name == other.name) && (self.age == other.age)
    }
}
pub fn perform_find_nth_element(){
    let person_elements = vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Charlie".to_string(), age: 22 },
    ];

    let nth_person = find_nth_element(&person_elements, 1);
    println!("Nth element for custom type: {:?}", nth_person);
}
//2-2

//3