use std::collections::HashMap;
use std::fmt::Formatter;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer,Serialize};
use serde::de::{Error, Visitor};


fn main() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    hobbies: Vec<String>,
    phone: Option<String>,
    gender: Gender,
    payment:Payment,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
enum Payment {
    CreditCard {
        card_number: String,
        expiration: String,
    },
    BankAccount {
        account_number: String,
        bank_name: String,
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Gender {
    Male,
    Female,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddressRequest {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
    #[serde(rename= "alamat")]
    address: AddressRequest,
}

#[derive(Serialize,Deserialize,  Debug)]
struct Admin {
    id: String,
    name: Name,
}

#[derive( Debug)]
struct Name {
    first: String,
    last: String,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let full_name = format!("{} {}", self.first, self.last);
        serializer.serialize_str(&full_name)
    }
}

impl <'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting name string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,

    {
        let result: Vec<&str> = v.split(" ").collect();
        if result.len() != 2 {
            return Err(E::custom("Expecting first and last name "));
        }
        Ok(Name {
            first: result[0].to_string(),
            last: result[1].to_string(),
        })
    }
}

impl <'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(NameVisitor)
    }
}

struct NameVisitor;

#[test]
fn test_custom_serialize() {
    let admin = Admin {
        id: "admin".to_string(),
        name: Name {
            first: "Eko".to_string(),
            last: "Khannedy".to_string(),
        },

    };

    let json: String = serde_json::to_string(&admin).unwrap();
    println!("{}", json);


    let result: Admin = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    id: String,
    name: String,
    #[serde(with= "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with= "chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>,
}

#[test]
fn test_chrono() {
    let category = Category {

    id: "123".to_string(),
    name: "Rust".to_string(),
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    };

    let json = serde_json::to_string(&category).unwrap();
    let result: Category = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_enum() {
    let user = User {
        username: "testUser".to_string(),
        email: "test@gmail.com".to_string(),
        gender: Gender::Male,
        hobbies: vec!["testHobbies".to_string(), "swimming".to_string(), "baca buku".to_string()],
        phone: Some("13221321-2323-3232".to_string()),
        payment: Payment::BankAccount {
            bank_name: "Bank BCA".to_string(),
            account_number: "131232132131".to_string(),
        }
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let result : User = serde_json::from_str(&json).unwrap();
    println!("{:?}", result.gender);
}



#[test]
fn test_vector() {
    let user = User {
        username: "testUser".to_string(),
        email: "test@gmail.com".to_string(),
        gender: Gender::Male,
        hobbies: vec!["testHobbies".to_string(), "swimming".to_string(), "baca buku".to_string()],
        phone: None,
        payment: Payment::BankAccount {
            bank_name: "Bank BCA".to_string(),
            account_number: "131232132131".to_string(),
        }
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let result : User = serde_json::from_str(&json).unwrap();
    println!("{:?}", result.payment);
}
#[test]
fn test_create_json_from_array() {
    let numbers = [10, 11, 12, 13,14];

    let json = serde_json::to_string(&numbers).unwrap();
    println!("json: {}", json);
}

#[test]
fn test_create_json_for_create_user_request() {
    let request = CreateUserRequest {
        username: "testUser".to_string(),
        password :"testpassword".to_string(),
        email: "test@gmail.com".to_string(),
        address : crate::AddressRequest {
            street: "12345 Main St".to_string(),
            city: "Spring Field".to_string(),
            state: "IL".to_string(),
            zip: "62701".to_string()
        }

    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);

}



#[test]
fn test_create_json_for_user_login_request() {
    let login_request = UserLoginRequest {
        username : "testuser".to_string(),
        password : "testpassword".to_string(),
    };

    let json = serde_json::to_string(&login_request).unwrap();
    println!("json: {}", json);

    let login_result: UserLoginRequest = serde_json::from_str(&json).unwrap();
    println!("login_result: {:?}", login_result);
}

#[test]
fn test_map() {
    let mut values: HashMap<String, i32> = HashMap::new();

    values.insert("one".to_string(), 1);
    values.insert("two".to_string(), 2);
    values.insert("three".to_string(), 3);

    let json = serde_json::to_string(&values).unwrap();
    println!("{}", json);

    let result : HashMap<String, i32> = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}
