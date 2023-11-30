# Rust Memory and Permissions

This README wraps up the most important concepts inside chapter 4 of the Rust Book.
Inside the `src` folder are put correspondent examples (work in progress).

## Operations on Memory

Sources [4.1 What Is Ownership?](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)
[4.2 References and Borrowing](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html)
[4.3 Fixing Ownership Errors](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html)

#### 1. Undefined behaviour

- **use-after-free** &rarr; it's possible to _have_ a pointer to freed memory but &rarr; it's not permitted to _use_ it (W)

  ```rust
  let my_vector = vec![1,2];
  let pointer_1 = &my_vector[0];

  my_vector.push(3); //pointer_1 has been freed
  println!("{pointer_1}");
  ```

- **double-free** &rarr; pointer used after its data its freed &rarr; moving out of a reference

  ```rust
  let my_vector = vec![1,2];
  let pointer_1 = &my_vector[0];

  let first_element = *pointer_1;
  drop(first_element); // vec![1,2] is also freed
  drop(my_vector); //error! vec![1,2] was already freed
  ```

- [Full list](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) 
  - memory corruption (70%)

#### 2. Frame

- it's a _mapping_ of **variables-values** inside a single function (scope)
- after a function returns, the frame is _deallocated_ -or _freed_ or _dropped_

#### 3. Stack

- contains list of frames of _currently-called-functions_
- LIFO (popped off)

#### 5. Heap
- heap data can persist beyond the scope of a single function call &rarr; it can be moved, shared, and referenced by different parts of the program. It's deallocated when there are no more reference to it.
- hosts 2 types of pointers:
  1. Boxes (pointers owning data on the Heap)
     - are constructs used by collections (Vec, String, HashMap)
  ```rust
      let box = Box::new(1) //box is a pointer
      let pointer_to_box = box; //copying a pointer
      // pointer_to_box is another pointer to 1
      // box has been moved
  ```
  1. references to the stack
  ```rust
  let ref_box_stack = &box
  ```
  1. references to the heap
  ```rust
  let ref_content_box_heap = &*box
  ```
- `push` reallocates vector data

#### 4. References

- e.g. `let pointer_to_the_heap = String::from("heap_string");` or `&v[2]`:
  - Non-owning pointer &rarr; it just _borrows_ a variable's data
  - **Pointer Safety Principles**
    1. data should not be aliased (R) and mutated (W) at the same time
    2. do not use any reference to a data, while another reference to the same data it's still alive
  - slices
  - IMMUTABLE REFERENCE (&)
    - ```rust
      let a = String::from("x")`;
      let b = &a;
      ```
    - the variable that hold the reference `b` has only (R) permission and creates temporary **aliasing** &rarr; accessing data through different variables
    - it _temporary removes WO (moving and borrowing) F permissions to the referenced data `a` `*b`_ until the variable that holds the reference `b` it's no longer alive (deallocation) &rarr; 
  - MUTABLE (UNIQUE) REFERENCE (&mut)
    - ```rust
      let a = String::from("x")`;
      let b = &mut a;
      ```
    - only one for each scope
    - the variable that holds the reference `b` has (RW) permissions on data `x`
    - it _temporary removes RWO (moving and borrowing) F permissions to the referenced data `a` `*b`_ until the variable that holds the reference `b` it's no longer alive (deallocation) &rarr; avoids undefined behaviour

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

     let c = add_some_stuff(a); //moving of ownership happening here: ownership is moved from a to b and a is freed!

     //! cannot access  {a} anymore: the pointer lost his connection to the heap

     fn add_some_stuff(mut b:String) -> String{
         b.push("_stuff")
         b
     }
     ```

  2. variable `a` can be assigned to the variable `b`:
     ```rust
     let a = Box:new(1);
     let b = a; //moving of ownership happening here: ownership is moved from a to b. A gets deallocated when not in use
     ```
  3. m. data `x` out of a reference `a` into `b`

  ```rust
  let a = x;
  let b = *a;
  ```

  - the variable `a`, that has lost ownership (it was freed), **cannot be used** after the moving of ownership
  - moving of ownership can be avoided using `.clone` &rarr; it creates _deep copies_ inside the heap
  - moving: Boxes and String (that do not implement Copy), when moved, are removed by all their permissions (RWO) (see HEAP)

- it can be **DROPPED** with `drop()`

#### Flow (F)

- checks safety of input/output references, that are treated differently than references within a function body

### Ownership common patterns

1. returning a reference to a function's local variable is not possible
2. passing immutable &ref as param and want to edit its content
3. using a reference while its data has been previously dropped by a function or taken by an alias
4. modifying array content i32 vs String
5. borrowing one element of the array and tuples

### Notes

- The borrow checker sometimes rejects programs that have defined behaviour &rarr; it does not check the content of variables or functions
  - tuple

##  Structs
### 1. Defining and Instantiating
#### 2. Intro
#### 3.  Field init shorthand
#### 4. Creating Instances from other instances with Struct update Syntax
#### 5. using tuple structs without namee fieds to create differnt types
#### 6. unit-like structs without any fields
#### 7. borrowing struct fields
