pub mod db;
pub mod services;

use crate::{
    db::check_or_create_table,
    services::establish_connection,
};
