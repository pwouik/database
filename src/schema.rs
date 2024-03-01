use crate::types::DataType;

pub struct TableInfo {
    pub name: String,
    pub data_type: DataType,
}
pub struct Schema {
    tables: Vec<TableInfo>,
}
impl Schema {
    pub fn new() -> Self {
        Schema { tables: vec![] }
    }
    pub fn table(&mut self, name: &str, data_type: DataType) -> &mut Self {
        self.tables.push(TableInfo {
            name: String::from(name),
            data_type,
        });
        self
    }
    pub fn get_tables(&self) -> &Vec<TableInfo> {
        &self.tables
    }
}
