use std::thread;

fn main() {
    variables_and_values();
    functions();
    ownership();
    life_time();
    borrowing();
    iterator();
    match_expressions();
    if_while_let_for();
    test_animals();
}

// basic enum:
enum UserRole { // convention: PascalCase type name
    Team1,      // convention: PascalCase enum name
    Team2,
    Managers,   // convention: comma after last list element (optional)
}

// enum with value and type:
// We will discuss typed enums and structs in the Object Oriented features.
// Option is part of standard lib, should not redefine it
enum MyOption<T> {
    None,
    Some(T),
}

// basic struct:
struct BasicUser1 {
    id: u32,    // convention: snake_case variable name
    active: bool,
}

// tuple struct (named tuple type):
struct IP4Address (u8, u8, u8, u8);

// struct with dynamic-size elements
struct User {
    id: u32,
    active: bool,
    name: String,
    roles: Vec<UserRole>,
}

static mut STARTUP_EPOCH_SECS: Option<i64> = None; // convention: globals are in UPPER_SNAKE_CASE
const ABC_DE: &str = "abc de"; // type must be explicit for static and const

fn variables_and_values() {
    let a: bool = true;               // bool
    let b = false;                    // inferred bool
    let c: u16 = 1;                   // u16
    let c_ptr = &c;                   // inferred &u16, reference to an u16 variable
    let mut c_copy = *c_ptr;          // inferred u16, de-reference c_ptr
    c_copy = 166;
    let d = 2 + 2;                    // inferred i32, because i32 is the default integer
    let e: f32 = 3.1415;              // f32
    let f = 13.5;                     // inferred f64, because f64 is the default float
    let g = 3 + c;                    // inferred u16, because c is u16
    let h = 0;                        // inferred usize, because it is later used as an array index, which must be usize
    let mut arr1: [i64; 2] = [1, 2];  // array of i64, length=3
    let i = arr1[h];                  // inferred i64, bacause arr1 is array of i64
    arr1[0] = 3;                      // array element is addressed with a 0-based index
    let arr2 = [1, 2, 3, 4];          // inferred mutable [i32; 4]
    let sli1 = &arr2[0..2];           // inferred &[i32] reference to an array slice ([1, 2])
    let tup1: (bool, u32) = (true, 0);// tuple of (bool, u32)
    let mut tup2 = (12, 3.14, "abc"); // inferred mutable tuple (i32, f64, &str)
    tup2.0 = 13;                      // tuple element is addressed with a 0-based index
    let j = '💖';                     // inferred char
    let str1: &str = "abcd";          // &str, reference to an str
    let sli2 = &str1[0..2];           // &str referring to slice of a string literal ("ab")
    let mut user = User {             // mutable structure variable
        id: 1,
        active: true,
        name: String::from("Joe"),    // create a new dynamic String from a literal.
                                      // Equivalent to "Joe".to_string()
        roles: vec![UserRole::Team2], // vec![] is a macro to initialise a Vec
    };
    user.active = false;              // update mutable structure
    user.name.push_str(" Smith");     // append to a String
    let localhost = IP4Address(127, 0, 0, 1);
    let first_byte = localhost.0;     // a tuple struct is adressed the same way as a tuple
    
}

// void function with a mutable argument, procedural style solution
// convention: snake_case function and argument names
fn search_pattern_for(pattern: &str, lines: &[&str], idx: &mut usize) {
    for i in 0..lines.len() {
        if lines[i].contains(pattern) {
            *idx = i;
            return;
        }
    }
    *idx = usize::MAX;
}


// function with a return value, FP style solution
// if there is no semicolon after the last line, it is considered a return value
// ("expr" is the same as "return expr;")
fn search_pattern_iter(pattern: &str, lines: &[&str]) -> usize {
    lines
        .iter()  // iterate over the elements,
                 // just like .stream() in Java (:Iterator<&str>)
        .enumerate() // extend each element with an index, as a tuple,
                     // just like .zip in Scala (:Iterator<(usize, &str)>)
        .find(|(_, &line)| line.contains(pattern)) // find the first element where the closure
                                                   // returns true (:Option<(usize, &str)>)
        .map_or(usize::MAX, |(idx, _)| idx) // take the index from the tuple, if found,
                                            // set MAX_USIZE otherwise (:usize)
}

// call example functions
fn functions() {
    let lines = ["abcde", "defgh", "ghijk"];
    let pattern = "gh";

    let mut idx: usize = usize::MAX;
    search_pattern_for(pattern, &lines, &mut idx);
    // println!() is a macro. Macros can have variable number of arguments,
    // functions must have fixed number of arguments
    println!("Matching line: {}", if idx < lines.len() {lines[idx]} else {"NOT FOUND"});

    let idx = search_pattern_iter(pattern, &lines);
    println!("Matching line: {}", if idx < lines.len() {lines[idx]} else {"NOT FOUND"});
}

fn return_match(pattern: &str, lines: Vec<&str>) -> Option<String> {
    lines
        .iter()
        .find(|&line| line.contains(pattern))
        .map(|&line| line.to_string())  // map &str to a String instance
}

// Ownership example
fn ownership() {
    let lines = vec!["abcde", "defgh", "ghijk"];
    let pattern = "gh";
    let line = return_match(pattern, lines);
    // at this point ownership of "lines" was transferred to the return_match() function
    // the scope of "lines" is ended, it cannot be used below this point 
}

fn return_match_borrow<'a>(pattern: &str, lines: &'a Vec<&str>) -> Option<&'a str> {
    lines
        .iter()
        .find(|&line| line.contains(&pattern))
        .map(|&line| line)
}

// Lifetime example
fn life_time() {
    let lines = vec!["abcde", "defgh", "ghijk"];
    let pattern = "gh";
    let line = return_match_borrow(pattern, &lines);
    // The ownership of "lines" is not transferred to the return_match() function
    // "lines" can be used below this point:
    let line0 = lines[0];
}

// Borrowing examples
fn borrowing() {
    // borrowing immutably:
    let list1 = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {:?}", list1);
    only_borrows();     // list is not changed and continues to be in scope

    // borrowing immutably:
    let mut list2 = vec![1, 2, 3];
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();  // list is updated, but continues to be in scope

    // taking ownership with the move keyword. This is mostly useful when passing a closure to a new thread:
    let mut list3 = vec![1, 2, 3];
    thread::spawn(move || {     // spawn fires up a new thread
                list3.push(4);
                println!("From thread: {:?}", list3);
            })                  // returns a JoinHandle
            .join()             // wait for the thread to finish and returns a Result<(), Error>
            .unwrap();          // unwrap Result: returns the Ok value or panics on Err

    // taking ownership automatically (fails compilation):
    let mut list = [(10, 1), (3, 5), (7, 12)];
    let mut sort_operations = vec![];
    let txt = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(txt.clone());  // the closure takes ownership of txt, it needs to be cloned every time
        r.0
    });

}

// iterator examples
fn iterator() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // Iterator adaptors are generic, Rust cannot infer the result type, we need to declare the type:
    let result: i32 = v1.iter()
        .map(|x| x + 1) // iterator adaptor
        .sum();         // consuming adaptor

    // Another choice: declare the result type on the adaptor:
    let result = v1.iter()
        .map(|x| x + 1)
        .sum::<i32>();
}

fn another_side_effect() {
    // do nothing...
}

// Pattern matching with "match"
fn match_expressions() {
    let msg = "ERROR";
    let option_int: Option<i32> = Some(42);
    let user = User { id: 1, active: true, name: String::from("Joe"), roles: vec![UserRole::Managers] };
    let array = [1, 2, 3];
    let num = 3;

    // match statement with literals:
    match msg {
        "ERROR" => println!("error!"), // single-line expressions are separated by comma
        txt => println!("{txt}!"),     // convention: comma after the last arm (optional)
    }                                  // no semicolon needed (but allowed)

    // match expression with enum and named variable:
    let double_val = match option_int {
        None => None,           // all 'arms' of the match must be covered, otherwise compiler error
        Some(n) => Some(2 * n), // n is the named variable
    };                          // semicolon is mandatory for expression, unless it is a return value

    // match statement with enum and value matching:
    match option_int {
        None => {}              // do-nothing arms has an open-close curly bracket
        Some(0) => println!("Zero is ignored!"),
        Some(n) => {            // multi-line expressions or statements are in curly brackets
            println!("n={}", n);
            another_side_effect();
        }                       // no comma needed after curly bracket (but allowed)
    }

    // match statement with struct values:
    // discarded values can be represented with _
    match user {
        User {id, active: true, name: _, roles: _} => println!("User {id} is active!"),
        User {id, active: false, name: _, roles: _} => println!("User {id} is inactive!"),
    }

    // match expression with array
    let array_starting_with_1 = match array {
        [1, _, _] => Some(array),
        _ => None,
    };

    // match statement with multiple patterns and ranges
    //
    match num {
        1 | 2 => println!("Small number"),  // multiple pattern
        3..=7 => println!("Medium number"), // range should be inclusive
        8..=9 => println!("Almost 10"),
        ..=0 => println!("Too small!"),     // we may use ..=N or N.. ranges
        _ => println!("Too big!"),
    }
}

fn transpose(&(x, y): &(i32, i32)) -> (i32, i32) {
    (y, x)
}

// Pattern matching with "let"
fn if_while_let_for() {
    let bread_spread = Some("butter");
    let mut stack = vec!['a', 'b', 'c'];

    // if let:
    if let Some(spread) = bread_spread {
        println!("The bread has {spread} on it");
    } else {
        println!("The bread is plain");
    }

    // while let:
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop
    for (index, value) in stack.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // tuple:
    let (a, b) = (2, true);

    let point = (2, 3);
    let transposed = transpose(&point);

}


trait Animal {
    fn name(&self) -> String;
    fn species(&self) -> String;
}

struct Fox(String);

struct Chicken(String);

impl Animal for Fox {
    fn name(&self) -> String {
        self.0.clone()
    }
    fn species(&self) -> String {
        "Fox".to_string()
    }
}

impl Animal for Chicken {
    fn species(&self) -> String {
        "Chicken".to_string()
    }
    fn name(&self) -> String {
        self.0.clone()
    }
}

// &dyn indicates that the type is a trait, not an object type
// the trait is implemented by Fox and Chicken trait objects
fn assert_animal(animal: &dyn Animal, name: &str, species: &str) {
    assert!(animal.name() == name);
    assert!(animal.species() == species);
}

fn test_animals() {
    let chicken = Chicken("Jenny".to_string());
    let fox = Fox("Joe".to_string());

    assert_animal(&chicken, "Jenny", "chicken");
    assert_animal(&fox, "Joe", "Fox");
}

use std::ops::Add;

// #[derive] is an annotation macro, it will auto-generate the implementation
// for the traits Debug, Copy, Clone and PartialEq
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn add_points() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}