pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: None }
    }

    pub fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let new_node_ptr = Box::into_raw(new_node);

        if self.tail.is_none() {
            self.head = unsafe { Some(Box::from_raw(new_node_ptr)) };
            self.tail = Some(new_node_ptr);
        } else {
            unsafe {
                (*self.tail.unwrap()).next = Some(Box::from_raw(new_node_ptr));
                self.tail = Some(new_node_ptr);
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head_value = head.value;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = None;
            }

            head_value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.dequeue() {}
    }
}
