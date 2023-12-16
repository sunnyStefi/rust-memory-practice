//1

//2-1


fn find_nth_element<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}
#[derive(Debug, Ord, Clone)] //todo
 struct Person {
        name: String,
        age: u32,
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