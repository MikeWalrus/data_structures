#![feature(iter_is_partitioned)]

pub mod cir_linked_list;
pub mod linked_list;
pub mod seq_list;

pub trait List<T>: FromIterator<T>
where
    for<'a> &'a Self: IntoIterator<Item = &'a T>,
{
    fn push(&mut self, elem: T);

    fn partition(self) -> Self
    where
        T: PartialOrd;
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    pub fn test_partition<L: List<i32>>()
    where
        for<'a> &'a L: IntoIterator<Item = &'a i32>,
    {
        let mut vecs: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![1, 0],
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 2, 2, 2],
            vec![1, 2, 3, 4],
            vec![5, 5, 5, 1],
            vec![4, 0, 0, 4],
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1, 5],
        ];

        let mut rng = rand::thread_rng();
        for _ in 1..1000 {
            vecs.push(
                (&mut rng)
                    .sample_iter(rand::distributions::Standard)
                    .take(100)
                    .collect(),
            );
        }

        let errors = vecs
            .into_iter()
            .map(|v| -> Result<(), (Vec<i32>, L)> {
                let first = *v.get(0).unwrap_or(&0);
                let mut l: L = v.clone().into_iter().collect();
                l = l.partition();
                let len = l.into_iter().count();
                (v.len() == len && l.into_iter().is_partitioned(|i| i < &first))
                    .then(|| ())
                    .ok_or((v, l))
            })
            .filter_map(Result::err)
            .map(|e| {
                print!("{:?} -> ", e.0);
                print!("[");
                for i in e.1.into_iter() {
                    print!("{}, ", i);
                }
                println!("]");
            })
            .next();
        assert!(errors.is_none())
    }
}
