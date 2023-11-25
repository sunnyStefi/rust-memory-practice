# Rust Memory and Permissions
This README wraps up the most important concepts inside chapters 4.1 and 4.2 of the Rust Book.
Inside the `src` folder are put correspondent examples (work in progress).

## Operations on Memory
Sources [4.1 What Is Ownership?](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)
[4.2 References and Borrowing](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html) 
[4.3 Fixing Ownership Errors](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html)

#### 1. Undefined behaviour
- it's possible to _have_ a pointer to freed memory but &rarr; it's not permitted to _use_ it
- [Full list here](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)

#### 2. Frame
- it's a _mapping_ of **variables-values** inside a single function (scope)
- after a function returns, the frame is _deallocated_ -or _freed_ or _dropped_

#### 3. Stack
- contains list of frames of _currently-called-functions_
- LIFO

#### 5. Heap
- hosts 2 types of pointers:
  1. Boxes (pointers owning data on the Heap)
     - constructs used by collections (Vec, String, HashMap)
     - data can only be accessed only by the owner, not aliasing
    ```rust
    let box = Box::new(1)
    ```
  2. references to the stack 
    ```rust
    let ref_box_stack = &box
    ```
  4. references to the heap
    ```rust
    let ref_content_box_heap = &*box
    ```
- `push` reallocates data
  
#### 4. References
- e.g. `let pointer_to_the_heap = String::from("heap_string");` or `&v[2]`:
    - Non-owning pointer &rarr; it just *borrows* a variable's data
    - **Pointer Safety Principles**
      1. data should not be aliased (R) and mutated (W) at the same time
      2. do not use any reference to a data, while another reference to the same data it's still alive
    - slices
    - IMMUTABLE REFERENCE (&)
        - the reference variable, `*reference` (R) creates temporary **aliasing** &rarr; accessing data through different variables
        - it *temporary removes WOF permissions to the referenced data* until the variable that's bound to the reference it's no longer alive (deallocation) &rarr; avoids undefined behaviour
    - MUTABLE (UNIQUE) REFERENCE (&mut)
        - the reference variable has RW permissions
        - it *temporary removes RWOF permissions to the referenced data* until the variable that's bound to the reference it's no longer alive (deallocation) &rarr; avoids undefined behaviour

#### 6. Deallocation
- deallocation is automatically managed by `rustc`
- when a variable on the Stack owns a Box and is deallocated,
  the Box:
  1. is also deallocated from the Heap
  2. is returned from borrowing

#### 7. Path Permissions on Data
Permissions are lost after a path|variable is not longer in use

#### Read (R)
- data can be copied to another location
- enabled by default
#### Write (W)
- data can be mutated in-place
- not enabled by default
#### Own (O) vs Garbage Collector
- enabled by default
- variables can **OWN** a data on the Heap (e.g. Box)
  ```rust
  let x = Box::new(1) //x owns the Box
  ```
- it can be **MOVED** from a variable `a` to another variable `b`, 
  - `rustc` looks at the whole type signature &rarr; different **tuple** type fields are considered just one type field

  1. variable `a` can be passed as parameter `b` to a function:

     ```rust
     let  a = String::from("heap_string_here"); //a is a POINTER to the heap

     let result = add_some_stuff(a); //moving of ownership happening here: ownership is moved from a to b

     //cannot access anymore {a}: the pointer lost his connection to the heap see later bullet **

     fn add_some_stuff(mut b:String) -> String{
         b.push("_stuff")
         b
     }
     ```

  2. variable `a` can be assigned to the variable `b`:

     ```rust
     let a = Box:new(1);
     let b = a; //moving of ownership happening here: ownership is moved from a to and a is freed
     ```
 
  - the variable `a`, that has lost ownership (it was freed), **cannot be used** after the moving of ownership
  - moving of ownership can be avoided using `.clone` &rarr; it creates _deep copies_ inside the heap
  - Boxes and String (that do not implement Copy), when moved, they seen removed all their permissions (RWO)

- it can be **DROPPED** with `drop()`
#### Flow (F)
- checks safety of input/output references, that are treated differently than references within a function body 


### Ownership common patterns
1. returning a reference to a function's local variable is not possible
2. passing immutable &ref as param and want to edit its content
3. using a reference while its data has been previously dropped by a function or taken by an alias
4. modifying array content i32 vs String
5. borrowing one element of the array and tuples
