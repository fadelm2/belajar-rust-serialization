use std::collections::HashMap;
use std::fs::create_dir;
use std::net::ToSocketAddrs;
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    hobbies: Vec<String>,
    phone: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
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
    address: AddressRequest,
}


#[test]
fn test_vector() {
    let user = User {
        username: "testUser".to_string(),
        email: "test@gmail.com".to_string(),
        hobbies: vec!["testHobbies".to_string(), "swimming".to_string(), "baca buku".to_string()],
        phone: Some("13221321-2323-3232".to_string())
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let result : User = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
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
