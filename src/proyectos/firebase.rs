/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          10-02-2023
    Titulo:         introducción a RUST
    Descripción:    Proyecto firebase
    Referencias:
    crate firebae:  https://crates.io/crates/firebase-rs

    Dependencias:   [dependencies]
                    firebase-rs = "2.0.6"
                    serde_json = "1.0.91"
                    serde = "1.0.152"
                    tokio = { version = "1.25.0", features = ["full"] }

    Licencia:

"https://nodeesp-d0b20-default-rtdb.firebaseio.com/"
KEY "yu2MQsnxJedOuqsNA48LOqTp8g7LE3BE5w5wNPK6"

https://www.youtube.com/watch?v=PbCBlOKcuOo
https://carlos.lat/blog/rust-firebase-crud/

***************************************************************************************/

use std::collections::HashMap;
use firebase_rs::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    name: String,
}

//*****************************************************************************
#[tokio::main]
#[allow(dead_code)]
pub async fn firebase() {

    let user     = User {
        name    :"Akhil Sharma".to_string(),
        age     :30,
        email   :"akhil.sharma@gmail.com".to_string(),
    };


    let firebase = Firebase::auth("https://nodeesp-d0b20-default-rtdb.firebaseio.com/","yu2MQsnxJedOuqsNA48LOqTp8g7LE3BE5w5wNPK6").unwrap();

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted!");

}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response{
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User>{
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User{
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User{
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String){
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// convert a string to a response
fn string_to_response(s: &str) -> Response{
    serde_json::from_str(s).unwrap()
}

//convert a string to a user
fn string_to_user(s: &str) -> User{
    serde_json::from_str(s).unwrap()
}

//*****************************************************************************