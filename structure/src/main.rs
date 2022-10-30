// 構造体を勉強するよー!!
// 2022.10.29 (高専祭1日目) p.97

#![allow(unused)]struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}