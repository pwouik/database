use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::mem;
use std::ops::Bound;
use ordered_float::OrderedFloat;
use crate::column::ColumnTrait;
use crate::selection::Selection;

pub enum Comp{
    Less,
    Greater
}

pub trait Index{
    fn build_from(&mut self,column:&dyn ColumnTrait);
    fn get_bounded(&self, comp: Comp, bound: Bound<&dyn Any>) ->Selection;
    fn get_value(&self, value:&dyn Any)->Selection;
}
fn as_any(r:&dyn Any)-> &dyn Any {
    r
}
fn convert_float<T:Clone + 'static>(v:&dyn Any)->T{
    if TypeId::of::<T>() == TypeId::of::<OrderedFloat<f32>>(){
        as_any(&OrderedFloat::<f32>::from(v.downcast_ref::<f32>().unwrap().clone())).downcast_ref::<T>().unwrap().clone()
    }
    else if TypeId::of::<T>() == TypeId::of::<OrderedFloat<f64>>() {
        as_any(&OrderedFloat::<f64>::from(v.downcast_ref::<f64>().unwrap().clone())).downcast_ref::<T>().unwrap().clone()
    }
    else{
        v.downcast_ref::<T>().unwrap().clone()
    }
}
impl<T:Ord + Clone + 'static> Index for BTreeMap<T,usize> {
    fn build_from(&mut self,column:&dyn ColumnTrait) {
        for i in 0..column.len(){
            self.insert(convert_float::<T>(column.get(i).as_ref()), i);
        }
    }
    fn get_bounded(&self, comp: Comp, bound: Bound<&dyn Any>) ->Selection {
        let b = match bound {
            Bound::Included(v) => {Bound::Included(convert_float::<T>(v))}
            Bound::Excluded(v) => {Bound::Excluded(convert_float::<T>(v))}
            Bound::Unbounded => {Bound::Unbounded}
        };
        match comp {
            Comp::Less => {
                Selection::new(self.range((Bound::Unbounded,b)).map(|(_,v)|v.clone()).collect())
            }
            Comp::Greater => {
                Selection::new(self.range((b,Bound::Unbounded)).map(|(_,v)|v.clone()).collect())
            }
        }
    }
    fn get_value(&self, value:&dyn Any)->Selection{
        if let Some(v)=self.get(&convert_float::<T>(value)){
            Selection::new(vec![*v;1])
        }
        else {
            Selection::empty()
        }
    }
}