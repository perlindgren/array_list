// triggers ICE

#[derive(Debug)]
struct Node<'a, T> {
    data: &'a T,
}
static mut A: [Node<Option<i32>>; 4] = [Node { data: &None }; 4];

fn main() {
    println!("{:?}", unsafe { A });
}
