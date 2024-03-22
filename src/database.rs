use std::any::Any;
use crate::column::Column;
use crate::schema::Schema;
use crate::table::Table;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::array_column::VecColumn;
use crate::types::DataType;

pub struct Database {
    tables: HashMap<String, Table>,
    path:Box<Path>
}

impl Database {
    pub fn new(path:&str) -> Self {
        Database {
            tables: HashMap::new(),
            path: Box::from(Path::new(path))
        }
    }
    pub fn open(path:&str)->Self{
        let mut db = Self::new(path);
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let dir_entry = path.unwrap();
            let mut file = File::open(dir_entry.path()).unwrap();
            let mut buf:Vec<u8>= vec![];
            file.read_to_end(&mut buf).unwrap();
            let mut i:usize =0;
            let mut table = Table::new();
            while i<buf.len(){
                let name_size = buf[i];
                i+=1;
                let name = String::from_utf8_lossy(&buf[i..i + name_size as usize]).to_string();
                i+=name_size as usize;
                let data_id = buf[i];
                i+=1;
                let mut column:Box<dyn Column> = match data_id {
                    1=>Box::new(VecColumn::<u8>::new(DataType::Byte)) ,
                    2=>Box::new(VecColumn::<i16>::new(DataType::Short)),
                    3=>Box::new(VecColumn::<i32>::new(DataType::Int)),
                    4=>Box::new(VecColumn::<i64>::new(DataType::Long)),
                    5=>Box::new(VecColumn::<f32>::new(DataType::Float)),
                    6=>Box::new(VecColumn::<f64>::new(DataType::Double)),
                    7=>{
                        i+=1;
                        Box::new(VecColumn::<String>::new(DataType::String(buf[i-1])))
                    }
                    _ => unreachable!()
                };
                let data_size = usize::from_le_bytes(buf[i..i+8].try_into().unwrap());
                i+=8;
                column.decode(&buf[i..i+data_size]);
                table.add_column(name,column);
                i+=data_size;
            }
            db.add_table(dir_entry.path().file_stem().unwrap().to_str().unwrap(),table);
        }
        db
    }
    pub fn add_new_table(&mut self, name: &str, shema: &mut Schema) {
        let mut table = Table::new();
        for t in shema.get_tables() {
            let column:Box<dyn Column> = match t.data_type {
                DataType::Byte=>Box::new(VecColumn::<u8>::new(DataType::Byte)) ,
                DataType::Short=>Box::new(VecColumn::<i16>::new(DataType::Short)),
                DataType::Int=>Box::new(VecColumn::<i32>::new(DataType::Int)),
                DataType::Long=>Box::new(VecColumn::<i64>::new(DataType::Long)),
                DataType::Float=>Box::new(VecColumn::<f32>::new(DataType::Float)),
                DataType::Double=>Box::new(VecColumn::<f64>::new(DataType::Double)),
                DataType::String(size)=>{
                    Box::new(VecColumn::<String>::new(DataType::String(size)))
                }
            };
            table.add_column(String::from(t.name.clone()), column);
        }
        self.tables.insert(String::from(name), table);
    }

    pub fn add_table(&mut self, name: &str, table:Table) {
        self.tables.insert(String::from(name), table);
    }
    pub fn insert(&mut self, table_name: &str, row: Vec<Box<dyn Any>>) {
        let table = self.tables.get_mut(table_name).unwrap();
        table.insert(row);

    }
    pub fn get<'a, 'b: 'a>(
        &'b mut self,
        table_name: &str,
        index: usize,
    ) -> Vec<Box<dyn Any>> {
        let table = self.tables.get_mut(table_name).unwrap();
        table.get(index)
    }
    pub fn commit(&self) {
        fs::create_dir_all(&self.path).unwrap();
        for (name, table) in &self.tables {
            table.commit(&self.path.join(name));
        }
    }
}
