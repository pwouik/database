use crate::param::*;
use crate::value_ref::ValueRef;
use crate::types::*;
pub struct Column {
    pub data_type: DataType,
    element_size: u32,
    pub data: Vec<u8>,
}
impl Column {
    pub fn new(data_type: DataType) -> Self {
        Column {
            data_type,
            element_size: type_size(data_type),
            data: vec![],
        }
    }
    pub fn add(&mut self, value: Param) {
        let mismatch = match &value.data_type {
            DataType::String(size) => {
                if let DataType::String(size2) = self.data_type {
                    if *size > size2 {
                        panic!("parameter string of size {size} is greater than column string type of size {size2}");
                    }
                    false
                } else {
                    true
                }
            }
            t => *t != self.data_type,
        };
        if mismatch {
            panic!(
                "parameter type {:?} is different than column type {:?}",
                value.data_type, self.data_type
            );
        }
        if let DataType::String(size) = value.data_type {
            self.data.push(size);
        }
        let new_len = self.data.len() + self.element_size as usize;
        self.data.extend_from_slice(&value.data);
        if self.data.len() < new_len {
            self.data.resize(new_len, 0);
        }
    }
    pub fn get(&self, index: usize) -> ValueRef {
        ValueRef {
            column: self,
            index,
        }
    }
}
