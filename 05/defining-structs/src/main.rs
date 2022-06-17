struct User {
    username: String,
    email: String,
    sign_in_count: u8,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // jsと同じように省略できる
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 構造体をインスタンス化
    let mut user = User {
        email: String::from("someone@examole.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // インスタンスが可変なら変更可能
    user.username = String::from("new user name");

    let user1 = build_user("hiroyuki@example.com", "hiroyuki");
    let user2 = User {
        email: String::from("aaa@example.com"),
        username: String::from("hiroyuki2"),
        ...user1, // スプレッド演算子？が使える
    }
}


