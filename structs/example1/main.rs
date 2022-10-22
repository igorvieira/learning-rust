struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  // let user2 = User {
  //   active: user1.active,
  //   username: user1.username,
  //   email: String::from("another@examole.com"),
  //   sign_in_count: user1.sign_in_count,
  // };

  let _user2 = User {
    email: String::from("another@examole.com"),
    ..user1
  };

  user1.email = String::from("anotheremail@example.com");
}

// fn build_user(email: String, username: String) -> User {
//   User {
//     email,
//     username,
//     active: true,
//     sign_in_count: 1,
//   }
// }