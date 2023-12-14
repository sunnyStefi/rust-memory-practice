// 1 remove duplication without using generics
// create a function with params

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

pub fn print_largest_number(){
    println!("--------- Generics ---------");
    let list1 = vec![1,7,3];
    let list2 = vec![54,2,6];
    println!("largest numbers {} {}",largest_number(&list1),largest_number(&list2));
}

// 2 use generics as params
// needs > implementation
fn largest_generic<T>(list: &[T])  -> &T {
    let mut largest = &list[0];
    for generic in list {
        // if generic > largest { 
        //     largest = generic;
        // }
    }
    largest
}

pub fn print_largest_generic(){
    let list1 = vec![34,2,4];
    let list2 = vec!['a','g','x'];
    println!("Largest generic are {} {}", largest_generic(&list1),largest_generic(&list2));
}
// 3 generics in struct / enum
// x and y have the same type
struct Point<T> {
    x: T,
    y: T
}
//T is NOT the same type for both Point and DifferentPoint
struct DifferentPoint<T,U> {
    x: T,
    y: U
}

//needs Debug implementation
pub fn use_generic_struct(){
    let my_point = Point {x: 1, y:3};
    // let not_valid_point = Point {x: 1, y:3.0};
    let my_different_point = DifferentPoint {x:1.0, y:3};
    // println!("Generic points {:#?} {:#?}", my_point, my_different_point);  //needs Debug implementation
}

// 4 in struct / enum methods
impl<T> Point<T> {
    fn return_x(&self) -> &T {
        &self.x
    }
}
// it can also implement methods with concrete types -> use diffeerent names
impl Point<f32> {
    fn return_power_for_numbers(&self) -> f32 {
        self.x.powi(2)
    }
}

// mixing up other generics inside method
impl<T,U> DifferentPoint<T,U> {
    fn return_another_point<A,B>(self, other: DifferentPoint<A,B>) -> DifferentPoint<T,B>{
        DifferentPoint {
            x: self.x,
            y: other.y
        }
    }
}

pub fn use_generic_struct_method() {
    let my_point = Point {x:4.0, y:2.2};
    let my_different_point1 = DifferentPoint {x: 'a', y: 77};
    let my_different_point2 = DifferentPoint {x: 'b', y: 78};
    println!("x is {}", my_point.return_x());
    println!("power of x {}", my_point.return_power_for_numbers());
    let mixed_point = my_different_point1.return_another_point(my_different_point2);
    println!("mixing different points {} {}", mixed_point.x, mixed_point.y);

}

//10.2 Traits: defining shared behaviour
pub struct Article {
    pub headline: String,
    pub content: String
}

pub struct Tweet {
    pub retweet: bool,
    pub content: String
}

trait Summary {
    fn summarize(&self) -> String;

    //no need to implement this
    fn default_example(&self) -> String {
        String::from("I'm a default content")
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} {}", self.headline, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.retweet, self.content)
    }
}

fn use_trait_as_param(traitt: &impl Summary){
    println!("{}",traitt.summarize())
}

pub fn use_trait(){
    let article = Article {
        headline: String::from("Important headline!"),
        content: String::from("Nothing impressive"),
    };
    println!("{}", article.summarize());
    println!("{}", article.default_example());
    use_trait_as_param(&article);
}

