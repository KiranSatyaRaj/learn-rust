use std::{io, collections::HashMap};
fn main() {
    let mut dep_emp :HashMap<String, Vec<String>>= HashMap::new();

    for _ in 0..3 {
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("unable to read line");

        let emp_name = s.split_whitespace()
            .nth(1)
            .unwrap()
            .to_string();
        let dep = s.split_whitespace()
            .nth(3)
            .unwrap()
            .to_string();
        println!("{}", emp_name);

        let v = dep_emp.entry(dep).or_insert(Vec::new());
        v.push(emp_name);
    }

    println!("Enter name of the department");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("unable to read line");

    let dep_name  = s.trim().to_string();
    dep_emp.get_mut(&dep_name).unwrap().sort();
    println!("All people in {dep_name} department are {:?}", dep_emp.get(&dep_name).unwrap());

    println!("List of all employees in the company");
    for (k, v) in &mut dep_emp {
        v.sort();
        println!("Department: {k} and it's employees are {:?}", v);
    }
}