use crate::column::Column;
use crate::types::DataType;

pub struct ValueRef<'a> {
    pub column: &'a Column,
    pub index: usize,
}
impl<'a> From<ValueRef<'a>> for u8 {
    fn from(value: ValueRef<'a>) -> Self {
        if value.column.data_type != DataType::Byte {
            panic!("Can't cast {:?} into a u8", value.column.data_type);
        }
        value.column.data[value.index]
    }
}
impl<'a> From<ValueRef<'a>> for i16 {
    fn from(value: ValueRef<'a>) -> Self {
        if value.column.data_type != DataType::Short {
            panic!("Can't cast {:?} into a i16", value.column.data_type);
        }
        i16::from_ne_bytes(
            value.column.data[value.index..value.index + 2]
                .try_into()
                .unwrap(),
        )
    }
}
impl<'a> From<ValueRef<'a>> for i32 {
    fn from(value: ValueRef<'a>) -> Self {
        if value.column.data_type != DataType::Int {
            panic!("Can't cast {:?} into a i32", value.column.data_type);
        }
        i32::from_ne_bytes(
            value.column.data[value.index..value.index + 4]
                .try_into()
                .unwrap(),
        )
    }
}

impl<'a> From<ValueRef<'a>> for i64 {
    fn from(value: ValueRef<'a>) -> Self {
        if value.column.data_type != DataType::Long {
            panic!("Can't cast {:?} into a i64", value.column.data_type);
        }
        i64::from_ne_bytes(
            value.column.data[value.index..value.index + 8]
                .try_into()
                .unwrap(),
        )
    }
}
impl<'a> From<ValueRef<'a>> for f32 {
    fn from(value: ValueRef<'a>) -> Self {
        if value.column.data_type != DataType::Float {
            panic!("Can't cast {:?} into a f32", value.column.data_type);
        }
        f32::from_ne_bytes(
            value.column.data[value.index..value.index + 4]
                .try_into()
                .unwrap(),
        )
    }
}

impl<'a> From<ValueRef<'a>> for f64 {
    fn from(value: ValueRef<'a>) -> Self {
        if value.column.data_type != DataType::Double {
            panic!("Can't cast {:?} into a f64", value.column.data_type);
        }
        f64::from_ne_bytes(
            value.column.data[value.index..value.index + 8]
                .try_into()
                .unwrap(),
        )
    }
}

impl<'a> From<ValueRef<'a>> for String {
    fn from(value: ValueRef<'a>) -> Self {
        match &value.column.data_type {
            DataType::String(_) => String::from_utf8(
                value.column.data
                    [value.index + 1..value.index + 1 + value.column.data[value.index] as usize]
                    .try_into()
                    .unwrap(),
            )
            .unwrap(),
            t => panic!("Can't cast {:?} into a String", t),
        }
    }
}
