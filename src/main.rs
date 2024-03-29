mod column;
mod codec;
mod database;
mod index;
mod row_set;
mod schema;
mod table;
mod types;
mod selection;

use std::collections::Bound;
use crate::database::Database;
use crate::index::Comp;
use crate::schema::Schema;
use crate::types::DataType;

fn main() {
    let mut db = Database::new("db");
    db.add_new_table("people",Schema::new()
        .table("name",DataType::String(100))
        .table("age",DataType::Short)
        .table("size",DataType::Float));

    db.insert("people",vec![Box::new("john".to_string())  ,Box::new(45i16),Box::new(162.5f32)]);
    db.insert("people",vec![Box::new("george".to_string()),Box::new(22i16),Box::new(185.1f32)]);
    db.insert("people",vec![Box::new("sam".to_string())   ,Box::new(25i16),Box::new(175.3f32)]);

    let selection = db.get_bounded("people","size",Comp::Less,Bound::Included(&180f32));
    let values = db.get("people",selection);
    for row in values{
        println!("{}", row[0].downcast_ref::<String>().unwrap());
    }
    db.commit();
}
