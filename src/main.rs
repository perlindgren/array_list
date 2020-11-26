use core::mem::MaybeUninit;

#[derive(Debug, Copy, Clone)]
struct Node<T>
where
    T: Copy + PartialOrd,
{
    data: MaybeUninit<T>,
    next: Option<u8>, // u8 to represent index
}

#[derive(Debug)]
struct List<'a, T>
where
    T: Copy + PartialOrd,
{
    head: Option<u8>,
    free: Option<u8>,
    nodes: &'a mut [Node<T>],
}

impl<'a, T> List<'a, T>
where
    T: Copy + PartialOrd,
{
    fn new(nodes: &'a mut [Node<T>]) -> Self {
        for n in 0..nodes.len() - 1 {
            nodes[n].next = Some(n as u8 + 1);
        }

        Self {
            head: None,
            free: Some(0),
            nodes,
        }
    }

    #[inline]
    fn alloc(&mut self) -> u8 {
        match self.free {
            Some(i) => {
                self.free = self.nodes[i as usize].next;
                i
            }
            _ => panic!("Free depleted"),
        }
    }

    #[inline]
    fn free(&mut self, i: u8) {
        let mut n = self.nodes[i as usize];
        n.next = self.free;
        self.free = Some(i)
    }

    #[inline]
    fn insert_sort(&mut self, v: T) {
        // allocate a new node and set value
        let i = self.alloc();
        unsafe { self.nodes[i as usize].data.as_mut_ptr().write(v) };

        // get a pointer to the head
        let mut n = &mut self.head;

        while let Some(e) = n {
            if unsafe {
                &*self.nodes[*e as usize].data.as_ptr() < &*self.nodes[i as usize].data.as_ptr()
            } {
                break;
            }
            n = &mut self.nodes[*e as usize].next;
        }

        // i is the node index to insert
        // should be inserted before n

        let n_next = *n;
        *n = Some(i);
        self.nodes[i as usize].next = n_next;
    }

    fn push(&mut self, v: T) {
        match self.free {
            Some(n) => {
                self.free = self.nodes[n as usize].next;
                let mut new_head = &mut self.nodes[n as usize];

                unsafe {
                    new_head.data.as_mut_ptr().write(v);
                }
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
                unsafe { new_free.data.as_ptr().read() }
            }
            None => panic!("List empty"),
        }
    }
}

use std::fmt;

impl<'a, T> fmt::Display for List<'a, T>
where
    T: fmt::Debug + Copy + PartialOrd,
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

static mut NODES: [Node<i32>; 3] = [Node {
    data: MaybeUninit::uninit(),
    next: None,
}; 3];

fn main() {
    let mut list = List::new(unsafe { &mut NODES });

    println!("list {:?}", list);
    println!("list {}", list);

    let _ = list.insert_sort(1);
    println!("list {:?}", list);
    println!("list {}", list);

    let _ = list.insert_sort(3);
    println!("list {:?}", list);
    println!("list {}", list);

    let _ = list.insert_sort(2);
    println!("list {:?}", list);
    println!("list {}", list);
}
