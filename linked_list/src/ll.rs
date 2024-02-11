struct Node<T> {
    data : T,
    next : Option<Box<Node<T>>>
}

pub struct LinkedList<T> {
    head : Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head : none }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next : self.head.take()
        });

        self.head = Some(new_node)
    }

    pub fn pop(& mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head= node.next;
            node.data
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}