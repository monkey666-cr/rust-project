use authentication::{get_users, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,

    /// Add a user.
    Add {
        /// username.
        username: String,

        /// user login password.
        password: String,

        // user role.
        admin: Option<bool>,
    },

    /// Delete a user
    Delete {
        username: String,
    },

    // Change password
    ChangePassword {
        username: String,
        password: String,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:<20?}", user.username, user.role);
    })
}

fn add_user(username: String, password: String, role: LoginRole) {
    let mut users = get_users();

    let user = User::new(&username, &password, role);

    users.insert(username, user);

    save_users(&users);
}

fn delete_user(username: String) {
    let mut users = get_users();

    if users.contains_key(&username) {
        let value = users.remove(&username);
        println!("删除用户: {}, {:?}", username, value);
    }

    save_users(&users);
}

fn change_password(username: String, password: String) {
    let mut users = get_users();

    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&password);

        save_users(&users);
    } else {
        println!("{username} dose not exist");
    }
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => {
            add_user(
                username,
                password,
                if admin.unwrap_or(false) {
                    LoginRole::Admin
                } else {
                    LoginRole::User
                },
            );
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword { username, password }) => {
            change_password(username, password);
        }
        None => {
            println!("Run with --help to see instruction.")
        }
    }
}
