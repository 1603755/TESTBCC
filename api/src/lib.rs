pub mod db;
pub mod services;

use crate::{
    db::check_or_create_table,
    services::{
        establish_connection, process_request, process_change_door, process_get_door, get_rfid_table
    }
};
