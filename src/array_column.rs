use std::any::{Any, type_name};
use crate::codec::Codec;
use crate::column::Column;
use crate::types::{DataType, type_size};

pub struct VecColumn<T> {
    data: Vec<T>,
    data_type:DataType
}
impl<T:Clone+Codec + 'static> VecColumn<T>{
    pub fn new(data_type: DataType) -> Self {
        VecColumn {
            data: vec![],
            data_type
        }
    }
}
impl<T:Clone+Codec + 'static> Column for VecColumn<T> {
    fn len(&self)->usize{
        self.data.len()
    }
    fn get_data_type(&self) -> DataType {
        self.data_type
    }
    fn decode(&mut self, data:&[u8]){
        let size = type_size(self.data_type) as usize;
        let mut i:usize =0;
        while i<data.len(){
            self.data.push(T::decode(&data[i..i+size]));
            i+=size;
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut result = vec![];
        for i in &self.data{
            result.extend_from_slice(&*i.encode());
        }
        result
    }

    fn add(&mut self, value: Box<dyn Any>) {
        self.data.push(value.downcast_ref::<T>().unwrap().clone());
    }
    fn get(&self, index: usize) -> Box<dyn Any> {
        Box::new(self.data[index].clone())
    }
    /// SAFETY: set must be reverse ordered
    unsafe fn delete_reverse_ordered_set(&mut self, set:&mut Vec<usize>){
        for i in set{
            self.data.swap_remove(*i);
        }
    }
}
