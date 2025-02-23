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

    // Reference

    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet3(&m1, &m2); // note the ampersands
    println!("{}", format!("{} {}", m1, m2));
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

