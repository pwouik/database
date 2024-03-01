use crate::types::DataType;
pub struct Param<'a> {
    pub data_type: DataType,
    pub data: &'a [u8],
}
impl<'a> From<&'a u8> for Param<'a> {
    fn from(value: &'a u8) -> Self {
        Param {
            data_type: DataType::Byte,
            data: bytemuck::bytes_of(value),
        }
    }
}

impl<'a> From<&'a i16> for Param<'a> {
    fn from(value: &'a i16) -> Self {
        Param {
            data_type: DataType::Short,
            data: bytemuck::bytes_of(value),
        }
    }
}
impl<'a> From<&'a i32> for Param<'a> {
    fn from(value: &'a i32) -> Self {
        Param {
            data_type: DataType::Int,
            data: bytemuck::bytes_of(value),
        }
    }
}
impl<'a> From<&'a i64> for Param<'a> {
    fn from(value: &'a i64) -> Self {
        Param {
            data_type: DataType::Long,
            data: bytemuck::bytes_of(value),
        }
    }
}
impl<'a> From<&'a f32> for Param<'a> {
    fn from(value: &'a f32) -> Self {
        Param {
            data_type: DataType::Float,
            data: bytemuck::bytes_of(value),
        }
    }
}
impl<'a> From<&'a f64> for Param<'a> {
    fn from(value: &'a f64) -> Self {
        Param {
            data_type: DataType::Double,
            data: bytemuck::bytes_of(value),
        }
    }
}
impl<'a> From<&'a str> for Param<'a> {
    fn from(value: &'a str) -> Self {
        if value.len() > 255 {
            panic!("Length of string in byte is greater than 255");
        }
        let size: u8 = value.len() as u8;
        Param {
            data_type: DataType::String(size),
            data: &value.as_bytes(),
        }
    }
}
