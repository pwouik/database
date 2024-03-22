use std::any::Any;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::column::Column;
use crate::types::*;

pub struct Table{
    columns:Vec<(String, Box<dyn Column>)>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            columns: vec![]
        }
    }
    pub fn add_column(&mut self, name: String, column: Box<dyn Column>) {
        self.columns.push((name.into(), column));
    }
    pub fn insert(&mut self, row: Vec<Box<dyn Any>>) {
        if row.len() != self.columns.len() {
            panic!("Row have {} elements, {} expected", row.len(), self.columns.len());
        }
        let mut i: usize = 0;
        for val in row {
            self.columns[i].1.add(val);
            i += 1;
        }
    }
    pub fn get<'a, 'b: 'a>(&'b mut self, index: usize) -> Vec<Box<dyn Any>> {
        (0..self.columns.len()).map(|i| {
            self.columns[i].1.get(index)
        }).collect()
    }

    pub unsafe fn delete_set(&mut self, set: &mut Vec<usize>) {
        set.sort_unstable();
        set.reverse();
        for (_,col) in &mut self.columns{
            unsafe{col.delete_reverse_ordered_set(set)}
        }
    }
    pub fn select(){

    }
    pub fn commit(&self, path:&Path){
        let mut file = File::create(path.with_extension("temp")).unwrap();
        let mut buf:Vec<u8> = vec![];
        for (name,col) in &self.columns{
            buf.push(name.len() as u8);
            buf.extend_from_slice(name.as_bytes());
            let data_type = col.get_data_type();
            buf.push(as_byte(data_type));
            if let DataType::String(size) = data_type{
                buf.push(size);
            }
            buf.extend((col.len() as u64 * type_size(data_type) as u64).to_le_bytes());
            let mut data = col.encode();
            data.resize(type_size(data_type) as usize,0);
            buf.extend(&data);
        }
        file.write(&buf).unwrap();
        drop(file);
        fs::rename(path.with_extension("temp"),path.with_extension("tbl")).unwrap();
    }
}