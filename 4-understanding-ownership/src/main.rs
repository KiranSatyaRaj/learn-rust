use std::rc::Rc;
use std::str;

fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn main() {

    // safe
    let x = true;
    read(x);

    // unsafe
    // read(x);
    // let x = true;

    // Variables live in the stack
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is {}", y);

    // Heap Data
    let a = Box::new([0; 1_000_000]); // Box allocates the data on heap
    /* manual memory management is not permitted

         let b = Box::new([0; 100]);
         free(b);
         assert!(b[0] == 0);

     */

    // Box's memory in the heap is deallocated when variable it's variable frame is deallocated
    let a = 4;
    make_and_drop_box();

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    // println!("{full}, originally {first}"); variable first can't be used after being moved

    // Cloning avoid moves
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");


    // 4.2 References and Borrowing
    let m1 = String::from("Hello");
    let m2 = String::from("World");

    /*
    this results in error as data of m1, m2 are moved to params of greet()
            greet(m1, m2);
         format!("{} {}", m1, m2);
     */

    // alternative

    let (m1_again, m2_again) = greet2(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("formatted s is: {s}");

    // References are non-owning pointers

    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet3(&m1, &m2); // note the ampersands
    println!("{}", format!("{} {}", m1, m2));

    // Dereferencing a Pointer Accesses its Data

    let mut x = Box::new(1);
    let a = *x;                     // *x reads the heap value, so a = 1
    println!("x is {x} and a is {a}");
    *x += 1;                             // *x on the left-side modifies the heap value
    println!("x is {x} and a is {a}");   // so x points to the value 2

    let r1 = &x;              // r1 points to x on the stack
    let b = **r1;                   // two dereferences to get us to the heap value

    let r2 = &*x;                  // r2 directly points to the value in the heap
    let c = *r2;                    // so only one dereferencing is needed to read it

    // implicit and explicit dereferencing

    let x = Box::new(-1);
    let x_abs_1 = i32::abs(*x);     // explicit dereference
    let x_abs_2 = x.abs();          // implicit dereference
    assert_eq!(x_abs_1, x_abs_2);

    let r = &x;
    let r_abs_1 = i32::abs(**r);    // explicit dereference (twice)
    let r_abs_2 = r.abs();             // implicit dereference (twice)
    assert_eq!(r_abs_1, r_abs_2);

    let s = String::from("Hello");
    let s_len_1 = str::len(&s);
    let s_len_2 = s.len();
    assert_eq!(s_len_1, s_len_2);

    // Rust avoids simultaneous aliasing and mutation
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    v.push(4);
    // references in rust provide temporary aliasing
    // println!("The value of num is: {}", *num); // cannot alias and mutate at the same time

    // Borrow Checker
    let mut v: Vec<i32> = vec![1, 2, 3]; // v = [R, W, O]
    let num: &i32 = &v[2];              // v = [R, !W , !O], num = [+R, -, +O], *num = [+R, -, -]
    println!("The third element is {}", num); // v = [R, +W, +O], num = [!R, -, !O], *num = [!R, -, -]
    v.push(4); // v = [!R, !W, !O]

    let x = 0; // x = [+R, -, +O]
    let mut x_ref: &i32 = &x; // x = [!R, -, !O], x_ref = [+R, +W, +O], *x_ref = [+R, -, -]
    println!("value of x ref is {} and x is {} and *x_ref is {}", x_ref, x, *x_ref);
    x_ref = &3;
    println!("value of x ref is {} and x is {} and *x_ref is {}", x_ref, x, *x_ref);

    // The Borrow Checker finds permission violations
    // These are immutable references or shared references

    let mut v = vec![1, 2, 3];
    let num = &v[2];
    v.push(4); // num will be invalidated by this push
    // println!("The value of num is {}", *num); // results in error

    // Mutable references or unique reference
    let mut v: Vec<i32> = vec![1, 2, 3]; // v = [+R, +W, +O]
    let num: &mut i32 = &mut v[2];  // v = [!R, !W, !O], num = [+R, -, +O], *num = [+R, +W, -]
    println!("value of *num {}", *num);
    // println!("value of v is {:?}", v); // error
    *num += 2;
    println!("value of num {}", num);
    v.push(4);
    println!("value in vector v are {:?}", v);

    // mutable to read-only references
    let mut v = vec![1, 2, 3];
    let  num = &mut v[2];
    let num2 = &*num;

    println!("{} {}", *num, *num2);

    // Permission are returned at the end of the reference's lifetime
    let mut x = 1; // x = [+R, +W, +O]
    let y = &x; // y = [+R, -, +O], *y = [+R, -, -] x = [+R, !W, !O]
    let z = *y; // z = [+R, -, +O] , y = [!R, -, !O], *y = [!R, -, -], x = [+R, +W, +O] // y lifetime ends here
    x += z;


    // Data must outlive all of its references
    let s = String::from("Hello");
    let s_ref = &s;

    drop(s); // drop expects s to have R and O but s_ref has O perm
    // println!("value of s_ref is {}", s_ref);

    // 4.3 Fixing Ownership Errors

    let name = vec![String::from("Ferris")];
    let first = &name[0];
    // here name cannot be mutated inside the function as it ref'd by first
    stringify_name_with_title(&name);
    println!("{}", first);

    let mut name = vec![String::from("Ferris")];
    let first = &name[0];
    // safe_stringify_name_with_title(&mut name);
    println!("{}", first);

    let v: Vec<String> = vec![String::from("Hello World")];
    let s_ref = &v[0];
    // let s = *s_ref;
    println!("{}", *s_ref);

    // If a value does not own heap data then it can be copied without a move
    // accessing data without move
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    println!("{s}!");

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let s = v.remove(0);
    println!("{s}!");

    // Mutating different tuple fields
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean"),
    );

    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean"),
    );
    // let first = get_first(&name);
    // name.1.push_str(", Esq");
    println!("{first} {}", name.1);

    // Mutating Different Array elements
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];

    *x += 1;

    println!("{a:?}");

    let mut a = [0, 1, 2, 3];
    // let x = &mut a[1];
    // let y = &a[2];
    // *x += *y;

    let mut a = [0, 1, 2, 3];
    // let (a_l, a_r) = a.split_at_mut(2);
    // let x = &mut a[1];
    // let y = &a_r[0];
    // *x += y;

    let s = String::from("hello");
    let s_ref = &s;
    println!("{s} {}", *s_ref);
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe{*x += *y};  // DO NOT DO THIS unless you know what you're doing!

    // 4.4 The Slice Type
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    s.clear();

    // better way
    let mut s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let s2 = &s;
    println!("{hello}");
    s.push_str(" How are you?");

    // Range Syntax
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let mut s = String::from("hello world");
    let word = first_Word(&s);
    s.clear();
    // println!("the first word is {}", word); // cannot reference slice of s as its cleared in the previous line

    // String Literals are Slices
    let mut s = "Hello world!";

    let my_string = String::from("hello world");

    // first_Word works on slices of 'String s, whether partial or whole
    let word =  first_Word(&my_string[0..6]);
    let word = first_Word(&my_string[..]);
    // first_Word also works on references  to String s, which are equivalent
    // to whole slices of String s
    let word = first_Word(&my_string);

    let my_string_literal = "hello world";

    // first_Word works on slices of string literals, whether partial or whole
    let word = first_Word(&my_string_literal[0..6]);
    let word = first_Word(&my_string_literal[..]);
    let word = first_Word(&my_string_literal);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}


fn first_Word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();  // an array of bytes

    for (i, &item)  in bytes.iter().enumerate() {  // an iterator over array of bytes
        if item == b' ' {  // b' ' is byte literal
            return i;
        }
    }
    s.len()
}

// unsafe
fn get_first(name: &(String, String)) -> &String {
    &name.0
}

// unsafe
// fn add_big_string(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s|s.len()).unwrap();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// safe
fn add_big_String(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s|s.len()).unwrap().clone();
    for s in src {
        dst.push(s.clone());
    }
}

// safe
fn add_Big_String(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s|s.len()).unwrap();
    let to_add: Vec<String> = src.iter().filter(|s|s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}

// safe and optimal
fn add_Big_string(dst: &mut Vec<String>, src: &[String]) {
    let largest: usize = dst.iter().max_by_key(|s|s.len()).unwrap().len();
    for s in src {
        if s.len() > largest {
            dst.push(s.clone());
        }
    }
}

// safe
fn name_with_Title(name: &Vec<String>) -> String {
    let mut full =  name.join(" ");
    // full.push(" Esq.");
    full
}
fn name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
fn safe_stringify_name_with_Title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq"));
    let full = name.join(" ");
    full
}

fn safe_stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq.")); // immutable reference
    let full = name.join(" ");
    full
}

// unsafe function
// fn return_a_String() -> &String {
//     let s = String::from("Hello World");
//     &s // reference to s ends here,
// } // s cannot live outside this function

// safe
fn return_a_string() -> String {
    let s = String::from("Hello World");
    s
}

// another safe function
// this applies if we never intend to change the string and heap allocation is unnecessary
fn return_a_String() -> &'static str { // value return by this function lives forever
    "Hello World"  // returning a string literal
}

// yet another safe function
fn return_String() -> Rc<String> {
    let s = Rc::new(String::from("Hello world")); // ref count = 1
    // here ref count = 2, one for s and another is for return Rc<String>
    Rc::clone(&s) // create a new reference to s as s will be invalid after this function
} // at this point ref count = 1 only Rc<String>

// safe function again
fn return_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}
fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    s_ref // [+R, +F]
}

// unsafe
fn stringify_name_with_title(name: &Vec<String>) -> String {
    // name.push(String::from("Esq.")); // immutable reference
    let full = name.join(" ");
    full
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];  // v = [R, -, !O], *v = [!R, !W, -] c = [+R, -, +O], *c = [+R, -, -]

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase(); // *v = [R, +W, -], v = [R, -, +O], *c = c = lost all perms, up = [+R, -, +O]

        v[0]  = up
    } else {
        // v = *v = gains all perms, *c = c = lost all perms

        println!("Already capitalized {:?}", v);
    }
}

fn greet3(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}", g1, g2);
}

fn greet2(g1: String, g2: String) -> (String, String) {
    println!("{g1} {g2}");
    (g1, g2)
}

fn greet(g1: String, g2: String) {
    println!("{g1} {g2}");
}
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn make_and_drop_box() {
    let b = Box::new([0; 100]);
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

