use std::thread;
//Diff Fn-closures:
//1 no params type annotation
//2 narrow context relevance
#[derive(Debug, PartialEq, Copy, Clone)]
enum Shirt {
    Pink,
    Yellow
}

struct Inventory {
    shirts: Vec<Shirt>,
}

impl Inventory {
    fn gift(&self, preference: Option<Shirt>) -> Shirt {
        preference.unwrap_or_else(|| self.most_stocked()) //closure as argument
    }

    fn most_stocked(&self) -> Shirt {
        let mut num_pink = 0;
        let mut num_yellow = 0;

        for shirt in &self.shirts {
            match shirt {
                Shirt::Pink => num_pink += 1,
                Shirt::Yellow => num_yellow += 1,
            }
        }

        if num_pink > num_yellow {
            Shirt::Pink
        } else {
            Shirt::Yellow
        }

    }
}

pub fn giveaway(){
    println!("----------closures----------");
    let inventory = Inventory {
       shirts:  vec![Shirt::Pink, Shirt::Pink, Shirt::Yellow]
    };
    let user_preference = Some(Shirt::Pink);
    let giveaway = inventory.gift(user_preference);
    println!("{:?}", giveaway)

}

pub fn thread_(){
    let list = vec![1,2,3];
    
    thread::spawn(move || println!("From thread {:?}", list)).join().unwrap(); //as argument again but giving ownership to thread
}

pub fn iterator_(){
    let list = vec![1,2,3];
    let mut iterator = list.iter(); // MUT calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. 

    let first_item = iterator.next(); //*consuming adaptors
    println!("{:?}", first_item.unwrap());

    let total : i32= iterator.sum();  //sum is * takes ownership of the iterator!
    println!("{total}");

    //iterators adaptors
    //because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.
    let list2 : Vec<_> = list.iter().map(|x| x + 1).collect(); 
    println!("{:?}", list2);

    let use_filter_result = use_filter(list, &3);
    println!("{:?}",use_filter_result);

}

fn use_filter(list: Vec<i32>, number: &i32) -> Vec<i32> {
    list.into_iter().filter(|x| x == number).collect()
}