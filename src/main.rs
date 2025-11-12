use gd::{
    core::types::{AccountId, UserId},
    entities::UserReference,
};

fn main() {
    let user = UserReference::builder()
        .id(UserId::new(16))
        .name("RobTop")
        .account_id(AccountId::new(71))
        .build();

    println!("{user} -> {user:?}");
}
