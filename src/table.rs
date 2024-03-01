use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::column::Column;
use crate::param::Param;
use crate::value_ref::ValueRef;
use crate::types::{DataType, type_id};

pub struct Table{
    columns:Vec<(String, Column)>,
}

impl Table {
    pub fn new()->Self{
        Table{
            columns:vec![]
        }
    }
    pub fn add_column(&mut self,name:String,column: Column){
        self.columns.push((name.into(),column));
    }
    pub fn insert(&mut self, row: Vec<Param>) {
        if row.len() != self.columns.len() {
            panic!("Row have {} elements, {} expected", row.len(), self.columns.len());
        }
        let mut i: usize = 0;
        for val in row {
            self.columns[i].1.add(val);
            i += 1;
        }
    }
    pub fn select<'a, 'b: 'a>(&'b mut self, index:usize)->Vec<ValueRef<'a>>{
        (0..self.columns.len()).map(|i|{
            self.columns[i].1.get(index)
        }).collect()
    }
    pub fn persist(&self,path:&Path){
        let mut f = File::create(path.with_extension("temp")).unwrap();
        let mut buf:Vec<u8> = vec![];
        for (name,col) in &self.columns{
            buf.push(name.len() as u8);
            buf.extend_from_slice(name.as_ref());
            buf.push(type_id(col.data_type));
            if let DataType::String(size) = col.data_type{
                buf.push(size);
            }
            buf.extend((col.data.len() as u64).to_le_bytes());
            buf.extend(&col.data);
        }
        f.write(&buf).unwrap();
        drop(f);
        fs::rename(path.with_extension("temp"),path.with_extension("tbl")).unwrap();
    }
}