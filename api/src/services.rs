extern crate diesel;
use serde::{Deserialize, Serialize};
use mysql::*;
use actix_web::{error, post, get, web, Error as ActixError, HttpResponse, Responder};
use mysql::prelude::Queryable;
use actix_cors::Cors;
//import fs
use std::fs;


pub const PASSWORD : &str = "root_password";
pub const USER : &str = "root";

pub fn establish_connection() -> Result<PooledConn, ActixError> {
    let database_url = format!("mysql://{}:{}@db:3306/mydatabase", USER, PASSWORD);
    let pool = Pool::new(database_url.as_str());
    if pool.is_err() {
        return Err(error::ErrorInternalServerError("Failed to create pool"));
    }
    let pool = pool.unwrap();
    let result = pool.get_conn();
    if result.is_err() {
        return Err(error::ErrorInternalServerError("Failed to get connection from pool"));
    }
    let conn = result.unwrap();
    Ok(conn)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub antennaPort: u32,
    pub epc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub mail: String,
    pub password: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rfid {
    pub id: u32,
    pub antennaPort: u32,
    pub epc: String,
    pub timestamp: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
    pub id: u32,
    pub porta1: u32,
    pub porta2: u32,
}

#[post("/request")]
async fn process_request(requests: web::Json<Vec<Request>>) -> Result<HttpResponse, ActixError> {
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let requests = requests.into_inner();
    let mut conn = conn.unwrap();
    for request in requests {
        let query = format!("INSERT INTO rfid_table (antennaPort, epc) VALUES ({}, '{}')", request.antennaPort, request.epc); 
        conn.query_drop(query).expect("Failed to insert data");
    }
    Ok(HttpResponse::Ok().body("OK"))
}

#[get("/rfid")]
pub async fn get_rfid_table () -> Result<HttpResponse, ActixError> {
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let mut conn = conn.unwrap();
    let query = format!("SELECT * FROM rfid_table");
    let result = conn.query_map(query, |(id, antennaPort, epc, timestamp)| {
        Rfid {
            id,
            antennaPort,
            epc,
            timestamp,
        }
    });

    if result.is_err() {
        return Err(error::ErrorInternalServerError("Failed to get data from database"));
    }
    let result = result.unwrap();
    let mut response = String::from("[");
    for rfid in result {
        response.push_str(&format!("{{\"id\": {}, \"antennaPort\": {}, \"epc\": \"{}\", \"timestamp\": \"{}\"}},", rfid.id, rfid.antennaPort, rfid.epc, rfid.timestamp));
    }
    response.pop();
    response.push_str("]");
    Ok(HttpResponse::Ok().body(response))
}

#[post("/door-change")]
pub async fn process_change_door  (door: web::Json<Door>) -> Result<HttpResponse, ActixError> {
    //If the door exxists, update the door
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let mut conn = conn.unwrap();
    let query = format!("SELECT * FROM door_registry WHERE id = {}", door.id);
    let result = conn.query_map(query, |(id, porta1, porta2)| {
        Door {
            id,
            porta1,
            porta2,
        }
    });
    if result.is_err() {
        return Err(error::ErrorInternalServerError("Failed to get data from database"));
    }
    let result = result.unwrap();
    if result.len() > 0 {
        if door.porta1 == 1 {
            let query = format!("UPDATE mydatabase.door_registry SET porta1 = {} WHERE id = {}", door.porta1, door.id);
            conn.query_drop(query).expect("Failed to update data");
        } else if door.porta2 == 1 {
            let query = format!("UPDATE mydatabase.door_registry SET porta2 = {} WHERE id = {}", door.porta2, door.id);
            conn.query_drop(query).expect("Failed to update data");
        } else {
            let query = format!("UPDATE mydatabase.door_registry SET porta1 = {}, porta2 = {} WHERE id = {}", door.porta1, door.porta2, door.id);
            conn.query_drop(query).expect("Failed to update data");
        }
        
    } else {
        let query = format!("INSERT INTO mydatabase.door_registry (id, porta1, porta2) VALUES ({}, {}, {})", door.id, door.porta1, door.porta2);
        conn.query_drop(query).expect("Failed to insert data");
    }
    Ok(HttpResponse::Ok().body("OK"))
}

#[get("/door")]
async fn process_get_door () -> Result<HttpResponse, ActixError> {
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let mut conn = conn.unwrap();
    let query = format!("SELECT * FROM door_registry");
    let result = conn.query_map(query, |(id, porta1, porta2)| {
        Door {
            id,
            porta1,
            porta2,
        }
    });
    if result.is_err() {
        return Err(error::ErrorInternalServerError("Failed to get data from database"));
    }
    let result = result.unwrap();
    let mut response = String::new(); // String::from("[");
    for door in result {
        response.push_str(&format!("{{\"porta1\": {}, \"porta2\": {}}},", door.porta1, door.porta2));
    }
    response.pop();
    //response.push_str("]");
    Ok(HttpResponse::Ok().body(response))
}

#[post("/login")]
async fn process_get_login(requests: web::Json<Login>) -> Result<HttpResponse, ActixError> {
    println!("Login: {}", requests.mail);
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let mut conn = conn.unwrap();
    //Append the new login 
    let query = format!("INSERT INTO login (mail) VALUES ('{}')", requests.mail);
    conn.query_drop(query).expect("Failed to insert data");
    let mut response = String::new(); // String::from("[");
    response.push_str(&format!("{{\"mail\": \"{}\"}}", requests.mail));
    //response.push_str("]");
    println!("Response: {}", response);

    let html = fs::read_to_string("./web/home/index.html").unwrap();
    
    Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}
    
