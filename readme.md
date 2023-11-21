# Rust Memory and Permissions

This README wraps up the most important concepts inside chapters 4.1 and 4.2 of the Rust Book.
Inside the `src` folder are put correspondent examples (work in progress).

## Operations on Memory
Source [4.1 What Is Ownership?](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)

#### 1. Undefined behaviour
- it's possible to *have* a pointer to freed memory but &rarr; it's not permitted to *use* it 
- [Full list here](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)

#### 2. Frame
-   it's a *mapping* of variables-values inside a single function (scope)
-   after a function returns, the frame is *deallocated* -or *freed* or *dropped*

#### 3. Stack
- contains list of frames of *currently-called-functions*
- LIFO

#### 4. Pointer
- it's a special variables e.g. ```let i_am_a_pointer_to_the_heap = String::from("heap_string_here");``` that describes a location in memory
- permits access to the data without copying it

#### 5. Heap
- hosts Boxes
  - are constructs
  - are used by collections (Vec, String, HashMap)
- can contain pointers to the Stack

#### 6. Deallocation
- deallocation is automatically managed by `rustc`
- when a variable on the Stack owns a Box and is deallocated, the Box is also deallocated from the Heap

#### 7. Ownership
  - variables can **OWN** a data on the Heap (e.g. Box)
    ```rust
    let x = Box::new(1) //x owns the Box
    ```
  - ownership can be **MOVED** from a variable `a` to another variable `b`
  
    1. variable `a` can be passed as parameter `b` to a function
   
        ```rust
        let  a = String::from("heap_string_here");
    
        let result = add_some_stuff(a); //moving of ownership happening here: ownership is moved from a to b

        //cannot access anymore {a}: the pointer lost his connection to the heap see later bullet **

        fn add_some_stuff(mut b:String) -> String{
            b.push("_stuff")
            b
        }
        ````
    2. variable `a` can be assigned to the variable `b`

        ```rust
        let a = Box:new(1);
        let b = a; //moving of ownership happening here: ownership is moved from a to b
        ```
    - the variable that has lost ownership (`a`) **cannot be used** after the moving of ownership **
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


