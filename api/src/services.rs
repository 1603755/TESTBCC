extern crate diesel;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use mysql::*;
use mysql::prelude::*;

pub const PASSWORD : &str = "root_password";
pub const USER : &str = "root";

pub fn establish_connection() -> Result<PooledConn, Error> {
    let database_url = format!("mysql://{}:{}@mysql_db:3306/mydatabase", USER, PASSWORD);
    let pool = Pool::new(database_url.as_str())?;
    pool.get_conn()
}

