use rust_code_evolution::*;

#[test]
fn create_user() {
    let user = User {
        username: String::from("xiaoming"),
        age: 18,
    };

    assert_eq!(user, User::new(String::from("xiaoming"), 18));
}
