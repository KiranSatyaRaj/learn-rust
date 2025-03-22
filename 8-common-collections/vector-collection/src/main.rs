use std::ops::Range;

fn main() {
    let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    // Updating vector
    v.push(4);
    v.push(3);
    v.push(8);
    v.push(887);

    // Reading elements from vector

    let third = &v[2];
    println!("The third element in v is : {third}");

    let third = v.get(2);
    if let Some(num) = &third {
        println!("The third element is {num}");
    }

    match third {
        Some(num) => println!("The third element is {num}"),
        None => println!("There is no third element")
    }

    v.push(3);
    println!("{:#?}", v);

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let first = &v[1];
    // v.push(3);          // this produces compiler error
    println!("The first element is {first}");

    // iterating over values in a vector

    for n_ref in &v {
        // n_ref type is &i32
        let plus_one = n_ref + 1;
        println!("{plus_one}");
    }

    for n_ref in &mut v {
        *n_ref += 42;
    }
    println!("{:?}", v);

    let mut v = vec![1, 2];
    let mut iter= v.iter();
    let n1 = iter.next().unwrap();
    let n2 = iter.next().unwrap();
    let end = iter.next();

    // iterating over vector without a pointer
    let mut iter: Range<usize> = 0 .. v.len();
    let i1: usize = iter.next().unwrap();
    let n1 = &v[i1];

    // using enum to store multiple types
    enum SpreadSheetCall {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
      SpreadSheetCall::Int(4),
      SpreadSheetCall::Float(10.42),
      SpreadSheetCall::Text(String::from("blue")),
    ];
}

// naive implementation
// results in compiler error
fn dup_in_place(v: &mut Vec<i32>) {
    for n_ref in v.iter() {
        v.push(*n_ref);
    }
}