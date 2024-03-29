pub trait Codec {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: &[u8]) -> Self;
}

impl Codec for u8 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn decode(data: &[u8]) -> Self {
        data[0]
    }
}
impl Codec for i16 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn decode(data: &[u8]) -> Self {
        Self::from_le_bytes(data.try_into().unwrap())
    }
}
impl Codec for i32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn decode(data: &[u8]) -> Self {
        Self::from_le_bytes(data.try_into().unwrap())
    }
}
impl Codec for i64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn decode(data: &[u8]) -> Self {
        Self::from_le_bytes(data.try_into().unwrap())
    }
}
impl Codec for f32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn decode(data: &[u8]) -> Self {
        Self::from_le_bytes(data.try_into().unwrap())
    }
}
impl Codec for f64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn decode(data: &[u8]) -> Self {
        Self::from_le_bytes(data.try_into().unwrap())
    }
}

impl Codec for String {
    fn encode(&self) -> Vec<u8> {
        let mut result = vec![self.len() as u8];
        result.extend_from_slice(self.as_bytes());
        result
    }
    fn decode(data: &[u8]) -> Self {
        let size = data[0] as usize;
        Self::from_utf8(data[1..size + 1].to_vec()).unwrap()
    }
}
