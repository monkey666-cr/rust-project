use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, path::Path};

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    let users = get_users();

    if let Some(user) = users.get(&username) {
        let pwd = hash_password(password);
        if user.password == pwd {
            return Some(LoginAction::Granted(user.role));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    // 列表的获取方式
    // if let Some(user) = users.iter().find(|&user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // }

    return None;
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
    Unkonw,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        User {
            username: username.to_string(),
            password: password.to_string(),
            role,
        }
    }
}

// pub fn get_users() -> [User<'static>; 2] {
//     [
//         User::new("admin", "123456", LoginRole::Admin),
//         User::new("chenrun", "123", LoginRole::User),
//     ]
// }

// pub fn get_users<'a>() -> Vec<User<'a>> {
//     vec![
//         User::new("admin", "123456", LoginRole::Admin),
//         User::new("chenrun", "123", LoginRole::User),
//     ]
// }

pub fn get_default_users<'a>() -> HashMap<String, User> {
    let mut res = HashMap::new();

    res.insert(
        "admin".to_string(),
        User::new("admin", "123456", LoginRole::Admin),
    );

    res.insert(
        "chenrun".to_string(),
        User::new("chenrun", "123", LoginRole::User),
    );

    res
}

pub fn save_users(users: &HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let mut users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();

        for (_, value) in users.iter_mut() {
            let pwd = hash_password(&value.password);
            value.password = pwd;
        }
        users
    } else {
        let users = get_default_users();

        save_users(&users);

        users
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(&password);

    format!("{:X}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        let res = greet_user("chenrun");

        assert_eq!("Hello chenrun", res);
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "123456").unwrap(),
            LoginAction::Granted(LoginRole::Admin)
        );
        assert_eq!(
            login("chenrun", "123").unwrap(),
            LoginAction::Granted(LoginRole::User)
        );
        assert_eq!(login("xxxx", "xx"), None);
        assert_eq!(login("xxx", "123"), None);
    }
}
