#[derive(Debug)]
struct Node<'a, T>
where
    T: Copy,
{
    data: &'a T,
    next: Option<u8>, // u8 to represent index
}

#[derive(Debug)]
struct List<'a, T>
where
    T: Copy,
{
    head: Option<u8>,
    free: Option<u8>,
    nodes: &'a mut [Node<'a, T>],
}

impl<'a, T> List<'a, T>
where
    T: Copy,
{
    fn new(nodes: &'a mut [Node<'a, T>]) -> Self {
        for n in 0..nodes.len() - 1 {
            nodes[n].next = Some(n as u8 + 1);
        }

        Self {
            head: None,
            free: Some(0),
            nodes,
        }
    }

    fn push(&mut self, v: &'a mut T) {
        match self.free {
            Some(n) => {
                self.free = self.nodes[n as usize].next;
                let mut new_head = &mut self.nodes[n as usize];

                new_head.data = v;
                new_head.next = self.head;
                self.head = Some(n);
            }
            None => panic!("List full"),
        }
    }

    fn pop(&mut self) -> T {
        match self.head {
            Some(n) => {
                self.head = self.nodes[n as usize].next;

                let mut new_free = &mut self.nodes[n as usize];

                new_free.next = self.free;
                self.free = Some(n);
                new_free.data.clone()
            }
            None => panic!("List empty"),
        }
    }
}

use std::fmt;

impl<'a, T> fmt::Display for List<'a, T>
where
    T: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List :");
        let mut n = self.head;
        while let Some(i) = n {
            write!(f, "{:?}, ", self.nodes[i as usize].data);
            n = self.nodes[i as usize].next;
        }
        Ok(())
    }
}

static mut NODES: [Node<Option<i32>>; 4] = [Node {
    data: &None,
    next: None,
}; 4];

fn main() {
    let mut list = List::new(unsafe { &mut NODES });
    list.push(&mut Some(1));
    list.push(&mut Some(2));
    list.push(&mut Some(3));
    let v = list.pop();
    println!("{:?}", v);

    list.push(&mut Some(5));

    println!("list {:?}", list);
    println!("list {}", list);

    let _ = list.pop();
    let _ = list.pop();
    let _ = list.pop();
    println!("list {:?}", list);
    println!("list {}", list);

    let _ = list.pop();
    println!("list {}", list);

    let _ = list.pop();
}
