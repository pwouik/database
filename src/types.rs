use std::any::TypeId;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DataType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String(u8),
}
pub fn as_byte(t: DataType) -> u8 {
    match t {
        DataType::Byte => 1,
        DataType::Short => 2,
        DataType::Int => 3,
        DataType::Long => 4,
        DataType::Float => 5,
        DataType::Double => 6,
        DataType::String(_) => 7,
    }
}

pub fn type_size(t: DataType) -> u32 {
    match t {
        DataType::Byte => 1,
        DataType::Short => 2,
        DataType::Int => 4,
        DataType::Long => 8,
        DataType::Float => 4,
        DataType::Double => 8,
        DataType::String(size) => size as u32 + 1,
    }
}
pub fn type_id(t: DataType) -> TypeId {
    match t {
        DataType::Byte => TypeId::of::<u8>(),
        DataType::Short => TypeId::of::<i16>(),
        DataType::Int => TypeId::of::<i32>(),
        DataType::Long => TypeId::of::<i64>(),
        DataType::Float => TypeId::of::<f32>(),
        DataType::Double => TypeId::of::<f64>(),
        DataType::String(_) => TypeId::of::<String>(),
    }
}
