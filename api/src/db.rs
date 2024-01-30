// src/db.rs
use diesel::prelude::*;
use std::ops::Deref;
use mysql::*;
use mysql::prelude::{*, Queryable};
use mysql::prelude::*;
use mysql::{Conn, OptsBuilder};
use actix_web::Error as ActixError;
use actix_web::error;

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

pub fn check_or_create_table() -> Result<(), ActixError> {
    let conn = establish_connection();
    if conn.is_err() {
        return Err(error::ErrorInternalServerError("Failed to connect Database"));
    }
    let mut conn = conn.unwrap();
    print!("Checking for table...");
    let create_table_query = r"
        CREATE TABLE IF NOT EXISTS rfid_table (
            id INT AUTO_INCREMENT PRIMARY KEY,
            antennaPort INT,
            epc VARCHAR(255),
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";
    // Execute the query
    conn.query_drop(create_table_query).expect("Failed to create table");

    println!("OK");
    print!("Checking for table...");
    let create_table_query = r"
        CREATE TABLE IF NOT EXISTS door_registry (
            id INT AUTO_INCREMENT PRIMARY KEY,
            porta1 INT,
            porta2 INT
        );
    ";
    conn.query_drop(create_table_query).expect("Failed to create table");
    println!("OK");

    let create_table_query = r"
        CREATE TABLE IF NOT EXISTS login (
            id INT AUTO_INCREMENT PRIMARY KEY,
            mail VARCHAR(255),
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";
    conn.query_drop(create_table_query).expect("Failed to create table");
    println!("OK");

    let create_table_query = r"
        CREATE TABLE IF NOT EXISTS logs_cafe (
            id INT AUTO_INCREMENT PRIMARY KEY,
            registry VARCHAR(255),
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";
    conn.query_drop(create_table_query).expect("Failed to create table");
    println!("OK");

    Ok(())
}
