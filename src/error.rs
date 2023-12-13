use std::fs::File;
use std::io::ErrorKind;

/* There are 2 types of errors:
1) Recoverable 
    - created by the user (e.g. file not found)
    - Result<T,E>
2) Unrecoverable situation
    - bugs (e.g. array out of bound)
    - panic! 
        1. calling explicitly
        2. making an action that creates a bug
    - print failure msg, unwind, stack cleaning, quit
*/ 


pub fn lets_panic(){
    println!("--------------Errors--------------");
    // panic!("some panic here please");
    let victor = vec![1,2,3];
    victor[2]; //victor[3] run with `RUST_BACKTRACE=1` cargo run environment variable to display a backtrace
}

pub fn lets_recover(){
    let file_reading_attempt = File::open("hello.txt");

    // 1 SIMPLE
    // let hello = match file_reading_attempt {
    //     Ok(file) => file,
    //     Err(err) => panic!("{:?}",err),
    // };

    // 2 HANDLE TYPE OF ERROR
    // let hello = match file_reading_attempt {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };

    // 3 unwrap_or_else

    // 4 unwrap
    // let file_or_panic = File::open("text.txt").unwrap();

    //5 expect is like unwrap but with panic message
    // let file_or_panic = File::open("text.txt").expect("Please add a text file in this project");

    //6 ? optional chaining operator: returns the type returned by the function (Some<&str> or None)

}

fn last_char_first_line(text: &str) -> Option<char> {
    //lines -> &str
    //next -> Some<&str> or None -> returns Option
    //chars -> iterator over characters
    //last -> Some<char> 
    text.lines().next()?.chars().last()
}