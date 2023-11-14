// src/db.rs
use diesel::prelude::*;
use crate::establish_connection;
use std::ops::Deref;
use mysql::*;
use mysql::prelude::{*, Queryable};
use mysql::prelude::*;
use mysql::{Conn, OptsBuilder};
use actix_web::Error as ActixError;
use actix_web::error;

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
    Ok(())
}
