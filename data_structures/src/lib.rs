#![feature(iter_is_partitioned)]
pub mod cir_linked_list;
pub mod linked_list;
pub mod seq_list;

pub trait List<T>: FromIterator<T>
where
    for<'a> &'a Self: IntoIterator<Item = &'a T>,
{
    fn push(&mut self, elem: T);

    fn partition(self) -> Self where T: Ord;
}
