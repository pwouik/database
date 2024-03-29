use crate::codec::Codec;
use crate::types::{type_size, DataType};
use std::any::{Any, type_name};
use crate::types::*;

pub trait ColumnTrait {
    fn len(&self) -> usize;
    fn get_data_type(&self) -> DataType;
    fn decode(&mut self, data: &[u8]);
    fn encode(&self) -> Vec<u8>;
    fn add(&mut self, value: &dyn Any);
    fn get(&self, index: usize) -> Box<dyn Any>;
    unsafe fn delete_reverse_ordered_set(&mut self, set: &mut Vec<usize>);
}
pub struct Column<T> {
    data: Vec<T>,
    data_type: DataType,
}
impl<T: Clone + Codec + 'static> Column<T> {
    pub fn new(data_type: DataType) -> Self {
        Column {
            data: vec![],
            data_type,
        }
    }
}
impl<T: Clone + Codec + 'static> ColumnTrait for Column<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
    fn get_data_type(&self) -> DataType {
        self.data_type
    }
    fn decode(&mut self, data: &[u8]) {
        let size = type_size(self.data_type) as usize;
        let mut i: usize = 0;
        while i < data.len() {
            self.data.push(T::decode(&data[i..i + size]));
            i += size;
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut result = vec![];
        let stride = type_size(self.data_type) as usize;
        for i in &self.data {
            let mut encoded = i.encode();
            encoded.resize(stride,0);
            result.extend_from_slice(&*encoded);
        }
        result
    }

    fn add(&mut self, value: &dyn Any) {
        self.data.push(value.downcast_ref::<T>().unwrap().clone());
    }
    fn get(&self, index: usize) -> Box<dyn Any> {
        Box::new(self.data[index].clone())
    }
    /// SAFETY: set must be reverse ordered
    unsafe fn delete_reverse_ordered_set(&mut self, set: &mut Vec<usize>) {
        for i in set {
            self.data.swap_remove(*i);
        }
    }
}
