mod utils;
use data_structures::{cir_linked_list::CirLinkedList, linked_list, seq_list::SeqList, List};

fn main() {
    let v: Vec<f64> = utils::read_numbers().unwrap();
    let l1: SeqList<&f64> = v.iter().collect();
    let l2: linked_list::LinkedList<&f64> = v.iter().collect();
    let l3: CirLinkedList<&f64> = v.iter().collect();

    let l1 = l1.partition();
    let l2 = l2.partition();
    let l3 = l3.partition();

    println!("Sequential List:");
    print_double(l1.into_iter());
    println!("Singly Linked List:");
    print_double(l2.into_iter());
    println!("Circular Doubly Linked List:");
    print_double(l3.into_iter());
}

fn print_double<'a, 'b: 'a, T>(i: T)
where
    T: Iterator<Item = &'a &'b f64>,
{
    for n in i {
        print!("{:.3e} ", n);
    }
    println!("\n")
}
