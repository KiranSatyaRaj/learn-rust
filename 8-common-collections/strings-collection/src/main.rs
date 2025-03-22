fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    // is the same as
    let s = "initial contents".to_string();
    // is the same as
    let s = String::from("initial contents");

    // valid code
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // updating a string

    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str doesn't take ownership
    let mut s2 = "baz";
    s.push_str(s2);
    println!("s2 is {s2}"); // this still works

    let mut s = String::from("lo");
    s.push('l');

    // concatenation

    // + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3= s1 + &s2;

    // add takes ownership of s1
    // it appends a copy of contents of s2 to s1
    // and returns back ownership of s1


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + &s2 + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // indexing into Strings
    // cannot index strings as it's unclear
    // whether the resultant value should be
    // a byte, scalar or a grapheme cluster

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{:?}", hello.chars());
}