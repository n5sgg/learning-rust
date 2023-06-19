use std::mem;

// Bad Singly-Linked Stack: http://rust-unofficial.github.io/too-many-lists/first.html
#[allow(dead_code)]
pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[allow(dead_code)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[allow(dead_code)]
struct Node {
    elem: i32,
    next: Link,
}
