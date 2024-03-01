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
fn ensure_16bit_boundaries(mut s:u32)->u32{
    if s<16 {
        s-=1;
        s|=s>>1;
        s|=s>>2;
        s|=s>>4;
        s|=s>>8;
        s+1
    }
    else{
        (s + 15) & !15
    }
}
pub fn type_id(t: DataType) -> u8 {
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
        DataType::String(size) => ensure_16bit_boundaries(size as u32 + 1),
    }
}