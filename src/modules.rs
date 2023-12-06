 mod my_module {
    pub fn hallo_world(){
        println!("Hallo");
    }
    
    pub mod nested_module{
        pub fn hello_world(){
            println!("Hello");
        }
    }
}

use my_module::nested_module; 
mod another_module{
    pub fn try_knit(){
        super::my_module::hallo_world(); //super path
    }
}

pub fn modules_(){
    crate::modules::my_module::hallo_world(); //absolute path
    my_module::hallo_world(); //relative path
    another_module::try_knit(); //using  super
    nested_module::hello_world(); //defining use - not inside another module (scope)

}


