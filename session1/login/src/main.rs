use authentication::{greet_user, login, LoginAction, LoginRole};

fn main() {
    println!("Hello, world!");

    let res = greet_user("chenrun");

    println!("{res}");

    loop {
        println!("请输入用户名:");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).unwrap();

        println!("请输入密码:");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).unwrap();

        match login(username.trim(), password.trim()) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Admin"),
                    LoginRole::User => println!("User"),
                    LoginRole::Unkonw => println!("None"),
                }
                break;
            }
            Some(LoginAction::Denied) => {
                println!("禁止登录");
            }
            None => {
                println!("New user system");
            }
        }
    }
}
