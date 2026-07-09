use std::fmt;

#[derive(Default)]
struct User {
    name: String,
    email: Option<String>,
    phone_number: Option<String>,
    age: Option<i32>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Info about User: {}", &self.name)?;
        match &self.email {
            Some(email) => write!(f, "User email: {}", email)?,
            None => (),
        }
        match &self.phone_number {
            Some(number) => write!(f, "User phone number: {}", number)?,
            None => (),
        }
        match &self.age {
            Some(age) => write!(f, "User age: {}", age)?,
            None => (),
        }
        Ok(())
    }
}
fn main() {
    let mut user: User = Default::default();
    let mut name: String = String::new();
    let mut email: String = String::new();
    let mut phone: String = String::new();
    let mut age_string: String = String::new();

    println!("Please write your username");
    'name: loop {
        match std::io::stdin().read_line(&mut name) {
            Ok(_) => {
                if name.trim() != "" {
                    user.name = name.clone();
                    break 'name;
                } else {
                    name = name.trim().to_string();
                }
            }
            Err(_) => (),
        }
        println!("Please write a valid name");
    }

    println!("Please enter your email, or leave empty for None");
    match std::io::stdin().read_line(&mut email) {
        Ok(_) => {
            if email.trim() != "" {
                user.email = Some(email);
            } else {
                user.email = None;
            }
        }
        Err(_) => user.email = None,
    }

    println!("Please enter your phone number, or leave empty for None");
    match std::io::stdin().read_line(&mut phone) {
        Ok(_) => {
            if phone.trim() != "" {
                user.phone_number = Some(phone);
            } else {
                user.phone_number = None;
            }
        }

        Err(_) => user.phone_number = None,
    }

    println!("Please enter you age, or leave empty for None");
    match std::io::stdin().read_line(&mut age_string) {
        Ok(_) => match age_string.trim().parse() {
            Ok(age) => user.age = Some(age),
            Err(_) => user.age = None,
        },
        Err(_) => user.age = None,
    }

    println!("{}", user);
}
