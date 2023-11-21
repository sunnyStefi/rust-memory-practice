# Rust Memory and Permissions

This readm wraps up the most important concept inside chapters 4.1 and 4.2 of the Rust Book.
Inside the `src` folder are put correspondent examples (in progress).

## 4.1 Ownership
Source [What Is Ownership?](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)
 🚸
### Operations on Memory

#### 1. Undefined behaviour
- e.g. undefined variable. [Full list here](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- it's possible to *have* a pointer to freed memory but &rarr; it's not permitted to *use* it 

#### 2. Frame
-   hosts variables
-   it's a *mapping* of variables-values inside a single *scope* (e.g. function)
-   after a function returns, the frame is *deallocated* (or *freed* or *dropped*)

#### 3. Stack
- contains frames of *currently-called-functions*
- LIFO

#### 4. Pointer
- is a special variables e.g. ```let i_am_a_pointer_to_the_heap = String::from("heap_string_here");```
- permits access the data without copying it
- describes a location in memory (Heap/Stack)

#### 5. Heap
- hosts Boxes
  - are constructs
  - can contain ponters to the Stack
  - are used by collections (Vec, String, HashMap)
- data can *outlive* a function: no automatic deallocation here (??)

#### 6. Deallocation
- memory management is not permitted: deallocation is automatically managed by rustc
- A variable owns a Box: when the variable's is deallocated in the Stack, the Box is also deallocated in the heap

#### 7. Ownership
  - variables can own a data on the heap (e.g. Box)
    ```rust
    let x = Box::new(1) //x owns the Box
    ```
  - can be **MOVED** to another variable
  
    1. variable `a` can passed as parameter `b` to a function
   
        ```rust
        let  a = String::from("heap_string_here");
    
        let result = add_some_stuff(a); //moving of ownership happening here: ownership is moved from a to b

        //cannot access anymore {a}: the pointer lost his connection to the heap

        fn add_some_stuff(mut b:String) -> String{
            b.push("_stuff")
            b
        }
        ````
    2. assignment

        ```rust
        let a = Box:new(1);
        let b = a; //moving of ownership happening here: ownership is moved from a to b
        ```
    - the variable that has lost ownership (`i_am_a_pointer_to_the_heap`,`a`) **cannot be used** after the moving of ownership
    - moving can be avoided using `.clone` &rarr; it createes *deep copies* inside the heap

## 4.2 References

### 0 Intro

### 1 R. are non-owning pointers

### 2 Dereferencing accesses its data

### 3 R. avoid simultaneous aliasing and mutation 

### 4 R. change permissions on paths

### 5 Borrow checker finds permissions violation

### 6 Mutable R. provide immutable non-owning access to data

### 7 Permissions are returned at the end of a R. lifetime

### 8 Data must outlive all its references


