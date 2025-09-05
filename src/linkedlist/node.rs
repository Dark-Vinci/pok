use std::ptr;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self{
            value,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl <T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Send> Sync for LinkedList<T> {}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self{
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.next = self.head;

        let new_node_ptr: *mut Node<T> = Box::into_raw(Box::new(new_node));

        if self.head.is_null() {
            self.head = new_node_ptr;
        } else {
            unsafe {
                (*self.head).prev = new_node_ptr;
            }
        }

        self.head = new_node_ptr;
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.prev = self.tail;

        let new_node_ptr: *mut Node<T> = Box::into_raw(Box::new(new_node));

        if self.tail.is_null() {
            self.tail = new_node_ptr;
        } else {
            unsafe {
                (*self.tail).next = new_node_ptr;
            }
        }

        self.tail = new_node_ptr;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let head = Box::from_raw(self.head);

            self.head = head.next;

            if self.head.is_null() {
                self.tail = ptr::null_mut();
            } else {
                (*self.head).prev = ptr::null_mut();
            }

            self.len -= 1;

            Some(head.value)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            let tail = Box::from_raw(self.tail);

            self.tail = tail.prev;

            if self.tail.is_null() {
                self.head = ptr::null_mut();
            } else {
                (*self.tail).next = ptr::null_mut();
            }

            self.len -= 1;

            Some(tail.value)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}