use crate::column::Column;
use crate::param::Param;
use crate::schema::Schema;
use crate::value_ref::ValueRef;
use crate::table::Table;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    pub fn add_table(&mut self, name: &str, shema: &mut Schema) {
        let mut table = Table::new();
        for t in shema.get_tables() {
            table.add_column(String::from(t.name.clone()), Column::new(t.data_type));
        }
        self.tables.insert(String::from(name), table);
    }
    pub fn insert(&mut self, table_name: &str, row: Vec<Param>) {
        let table = self.tables.get_mut(table_name).unwrap();
        table.insert(row);

    }
    pub fn select<'a, 'b: 'a>(
        &'b mut self,
        table_name: &str,
        index: usize,
    ) -> Vec<ValueRef<'a>> {
        let table = self.tables.get_mut(table_name).unwrap();
        table.select(index)
    }
    pub fn persist(&self) {
        fs::create_dir_all(&self.path).unwrap();
        for (name, table) in &self.tables {
            table.persist(&self.path.join(name));
        }
    }
}
