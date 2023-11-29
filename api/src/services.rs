extern crate diesel;
use serde::{Deserialize, Serialize};
use mysql::*;
use actix_web::{error, post, get, web, Error as ActixError, HttpResponse};
use mysql::prelude::Queryable;
use actix_cors::Cors;

pub const PASSWORD : &str = "root_password";
pub const USER : &str = "root";

pub fn establish_connection() -> Result<PooledConn, ActixError> {
    let database_url = format!("mysql://{}:{}@mysql_db:3306/mydatabase", USER, PASSWORD);
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

pub async fn process_change_door  (door: web::Json<Door>) -> Result<HttpResponse, ActixError> {
    //If the door exxists, update the door
    println!("a");
    let conn = establish_connection();
    println!("b");
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
    println!("c");
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
        println!("e");
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