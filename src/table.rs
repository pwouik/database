use crate::column::ColumnTrait;
use crate::types::*;
use crate::index::*;
use std::any::{Any, type_name, type_name_of_val, TypeId};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Bound;
use std::path::Path;
use ordered_float::OrderedFloat;
use crate::selection::Selection;


pub struct Field{
    pub name:String,
    pub column:Box<dyn ColumnTrait>,
    pub index:Option<Box<dyn Index>>
}
pub struct Table {
    columns: Vec<Field>,
}

impl Table {
    pub fn new() -> Self {
        Table { columns: vec![] }
    }
    pub fn add_column(&mut self, name: String, column: Box<dyn ColumnTrait>) {
        self.columns.push(Field{name:name.into(), column,index:None});
    }
    pub fn insert(&mut self, row: Vec<Box<dyn Any>>) {
        if row.len() != self.columns.len() {
            panic!(
                "Row have {} elements, {} expected",
                row.len(),
                self.columns.len()
            );
        }
        let mut i: usize = 0;
        for val in row {
            self.columns[i].column.add(val.as_ref());
            i += 1;
        }
    }
    pub fn create_index(&mut self, column_name:&str){
        for field in &mut self.columns {
            if field.name == String::from(column_name){
                dbg!(field.column.get_data_type());
                let mut index:Box<dyn Index>= match field.column.get_data_type(){
                    DataType::Byte => {Box::new(BTreeMap::<u8,usize>::new())}
                    DataType::Short => {Box::new(BTreeMap::<i16,usize>::new())}
                    DataType::Int => {Box::new(BTreeMap::<i32,usize>::new())}
                    DataType::Long => {Box::new(BTreeMap::<i64,usize>::new())}
                    DataType::Float => {Box::new(BTreeMap::<OrderedFloat<f32>,usize>::new())}
                    DataType::Double => {Box::new(BTreeMap::<OrderedFloat<f64>,usize>::new())}
                    DataType::String(_) => {Box::new(BTreeMap::<String,usize>::new())}
                };
                for i in 0..field.column.len(){
                    index.build_from(field.column.as_ref());
                }
                field.index = Some(index);
            }
        }
    }
    pub fn get(& mut self, selection: Selection) -> Vec<Vec<Box<dyn Any>>> {

        selection.indices.into_iter().map(|i|{ (0..self.columns.len())
            .map(|j| self.columns[j].column.get(i))
            .collect()}).collect()
    }

    pub unsafe fn delete_selection(&mut self, selection: &mut Selection) {
        selection.indices.sort_unstable();
        selection.indices.reverse();
        for field in &mut self.columns {
            // Set is reverse ordered
            unsafe { field.column.delete_reverse_ordered_set(&mut selection.indices) }
        }
    }
    pub fn get_bounded(&mut self, column_name:&str, comp: Comp, bound: Bound<&dyn Any>)-> Selection{
        let mut i:usize=0;
        for field in &self.columns {
            if field.name == String::from(column_name) {
                break
            }
            i+=1;
        }
        if self.columns[i].index.is_none(){
            self.create_index(column_name);
        }
        self.columns[i].index.as_ref().unwrap().get_bounded(comp,bound)
    }
    pub fn get_value(&mut self, column_name:&str,value:&dyn Any)-> Selection{
        let mut i:usize=0;
        for field in &self.columns {
            if field.name == String::from(column_name) {
                break
            }
            i+=1;
        }
        if self.columns[i].index.is_none(){
            self.create_index(column_name);
        }
        self.columns[i].index.as_ref().unwrap().get_value(value)
    }
    pub fn commit(&self, path: &Path) {
        let mut file = File::create(path.with_extension("temp")).unwrap();
        let mut buf: Vec<u8> = vec![];
        for field in &self.columns {
            buf.push(field.name.len() as u8);
            buf.extend_from_slice(field.name.as_bytes());
            let data_type = field.column.get_data_type();
            buf.push(as_byte(data_type));
            if let DataType::String(size) = data_type {
                buf.push(size);
            }
            buf.extend((field.column.len() as u64 * type_size(data_type) as u64).to_le_bytes());
            let data = field.column.encode();
            buf.extend(&data);
        }
        file.write(&buf).unwrap();
        drop(file);
        fs::rename(path.with_extension("temp"), path.with_extension("tbl")).unwrap();
    }
}
