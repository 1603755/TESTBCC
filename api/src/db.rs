// src/db.rs
use diesel::prelude::*;
use crate::establish_connection;
use std::ops::Deref;
use mysql::*;
use mysql::prelude::{*, Queryable};

pub fn check_or_create_table() -> Result<(), Error> {
    let mut conn = establish_connection()?;
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )")?;
    Ok(())
}
