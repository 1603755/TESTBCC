extern crate diesel;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use mysql::*;
use mysql::prelude::*;
use actix_web::{error, post, web, App, Error as ActixError, HttpResponse};
use futures::StreamExt;

// src/db.rs
use diesel::prelude::*;
use std::ops::Deref;
use mysql::*;
use mysql::prelude::{*, Queryable};
use mysql::prelude::*;
use mysql::{Conn, OptsBuilder};

//use system_time::SystemTime;
use std::time::SystemTime;


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
    pub door1: u32,
    pub door2: u32,
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

#[post("/door-change")]
async fn process_change_door  (doors: web::Json<Door>) -> Result<HttpResponse, ActixError> {
    //If the door exxists, update the door
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let mut conn = conn.unwrap();
    let query = format!("SELECT * FROM door_table WHERE id = {}", doors.id);
    let result = conn.query_map(query, |(id, door1, door2)| {
        Door {
            id,
            door1,
            door2,
        }
    });
    if result.is_err() {
        return Err(error::ErrorInternalServerError("Failed to get data from database"));
    }
    let result = result.unwrap();
    if result.len() > 0 {
        let query = format!("UPDATE door_table SET door1 = {}, door2 = {} WHERE id = {}", doors.door1, doors.door2, doors.id);
        conn.query_drop(query).expect("Failed to update data");
    } else {
        let query = format!("INSERT INTO door_table (id, door1, door2) VALUES ({}, {}, {})", doors.id, doors.door1, doors.door2);
        conn.query_drop(query).expect("Failed to insert data");
    }
    Ok(HttpResponse::Ok().body("OK"))
}

