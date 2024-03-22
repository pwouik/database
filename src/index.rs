use std::ops::RangeBounds;
use crate::column::Column;
use crate::param::Param;

trait Index{
    fn select<'a, R: RangeBounds<Param<'a>>>(range:R) ->Vec<usize>;
    fn find(value:Param)->usize;
}