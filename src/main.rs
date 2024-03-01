mod column;
mod database;
mod param;
mod schema;
mod value_ref;
mod table;
mod types;

use crate::database::Database;
use crate::types::DataType;
use crate::schema::Schema;
fn main() {
    let mut db = Database::new("db");
    db.add_table(
        "people",
        Schema::new()
            .table("name", DataType::String(100))
            .table("age", DataType::Short)
    );

    db.insert("people", vec!["john".into(), (&25i16).into()]);

    let mut r = db.select("people", 0);

    println!("name: {}", String::from(r.remove(0)));
    println!("age: {}", i16::from(r.remove(0)));
    db.persist();
}
