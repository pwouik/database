use std::any::Any;
use crate::types::*;
pub trait Column{
    fn len(&self)->usize;
    fn get_data_type(&self) -> DataType;
    fn decode(&mut self, data:&[u8]);
    fn encode(&self)->Vec<u8>;
    fn add(&mut self, value: Box<dyn Any>);
    fn get(&self, index: usize) -> Box<dyn Any>;
    unsafe fn delete_reverse_ordered_set(&mut self, set:&mut Vec<usize>);
}
