mod column;
mod database;
mod schema;
mod table;
mod types;
mod index;
mod row_set;
mod btree_index;
mod array_column;
mod codec;

use crate::database::Database;
use crate::schema::Schema;
use crate::types::DataType;

fn main() {
    let mut db = Database::open("db");
    let mut r = db.get("people",0);
    println!("name: {}",r.remove(0).downcast::<String>().unwrap());
    println!("age: {}", r.remove(0).downcast::<i16>().unwrap());
    db.commit();
}
