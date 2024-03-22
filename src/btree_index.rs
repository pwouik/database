use std::collections::BTreeMap;

struct BtreeIndex{
    index:BTreeMap<Box<[u8]>, usize>
}