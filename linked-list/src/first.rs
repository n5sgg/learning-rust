// Bad Singly-Linked Stack: http://rust-unofficial.github.io/too-many-lists/first.html
#[allow(dead_code)]
pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
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
    next: List,
}

