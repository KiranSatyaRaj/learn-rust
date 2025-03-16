// Each struct you define is its own type
struct User {
    active: bool,
    // These won't work until you specify lifetimes
    // for username and email (will be covered in chapter 10)
    // username: &str,
    // email: &str,
    username : String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;


fn main() {
    let mut user1 = User {
        email: String::from("spike@rust.com"),
        username: String::from("SpikeSpiegel2047"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("SpikeJulia@syn.com");

    let mut user2 = build_user(String::from("JetBlack2025"), String::from("jet.black@spf.com"));
    println!("{} {}", user2.email, user2.username);

    user2.username = user1.username;
    println!("{} {}", user2.email, user2.username);
    // println!("{} {}", user1.email, user1.username);  // cannot user as it's moved to user2 in line 20
    user1.username = String::from("SpikeJetBebop");
    println!("{} {}", user2.email, user2.username);
    println!("{} {}", user1.email, user1.username);

    let user3 = User {
        email: String::from("Faye.valentine@earth.com"),
        ..user2 // remaining fields values will be set to value of user2
    };

    // println!("{} {}", user2.email, user2.username); cannot use user2.username as it's moved to user3 on line 3

    let color = Color(0, 255, 0);
    let point = Point(-20, 10, 0);

    let subject = AlwaysEqual;


}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}