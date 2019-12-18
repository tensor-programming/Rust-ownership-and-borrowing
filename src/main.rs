/*
    Borrowing and Ownership in Rust
*/

/*
  Stack
  - Extremely Fast
  - Values must have Fixed Sizes
  - Always puts data in on Top
  - data pushed down as new data comes in
*/

/*
  Heap
  - Less Organized and Slower
  - Accepts Dynamicly Sized Data or Data that can Grow
  - Returns a Pointer which goes on the stack
  - Pointer points to where the data is on the Heap
*/

/*
Ownership Rules:

1) Each value has a variable which is its owner.

2) There can only be one owner at any given time.

3) When the owner goes out of scope, the value will be dropped out of memory.
*/

/*
Borrowing Rules:
    1) Allowed infinite borrows for readonly access.
    2) Readonly borrows make the original data immutable for their duration.
    3) Only allowed to pass one borrow at a time for write access/mutability.
*/

/* Rust Stack | Copy Types
    bool
    character
    numbers
    slices
    fix sized arrays
    tuples containing primitives
    function pointers
*/

fn main() {
    let mut s = String::from("A String");

    let mut a = &s[1..3];
    let b = &s[2..5];

    let c = &mut a;
    let d = &b;

    let y = false;
    let x = 10;

    own_and_borrow_stuff(&mut s, &y, x);
}

fn own_and_borrow_stuff(a: &mut String, b: &bool, c: u8) {}

// fn main() {
//     let mut a = String::from("A String");
//     {
//         let b = &a;
//         let c = &a;
//         let d = &a;

//         println!("{}, {}, {}", b, c, d);
//     }
//     a.pop();
//     {
//         let x = &mut a;
//         x.pop();
//     }
//     println!("{}", a);
// }

/*
    main
    ptr: a -> ptr: 0
              len: 8
              cap: 8

         b -> ptr: 15
              len: 8
              cap: 8

*/

/*
    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    |"A"|" "|"S"|"t"|"r"|"i"|"n"|"g"|
*/

/*
    global frame
    ptr main -> main()
*/
// fn main() {
//     let x = 10;
//     let y = x;
//     let z = x;

//     copy(true);
//     copy("a");
//     copy("a slice");
//     copy(x);
//     copy(String::from("Test"));
// }

// fn copy<T>(t: T) -> T
// where
//     T: Copy,
// {
//     let x = t;
//     x
// }

/*
    main
    x = 10
    y = 10
    z = 10


*/

/*
    main
    ptr: a -> ptr: 0
              len: 8
              cap: 8

    a.pop()
         a -> ptr: 0
              len: 7
              cap: 8

*/

/*
    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    |"A"|" "|"S"|"t"|"r"|"i"|"n"|"g"|
*/
