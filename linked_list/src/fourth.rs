//! 使用 unsafe 实现的队列，支持 push，pop，peek，iterator

pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::into_raw(Box::new(Node {
            elem,
            next: std::ptr::null_mut(),
        }));

        if self.tail.is_null() {
            self.head = new_node;
        } else {
            unsafe {
                (*self.tail).next = new_node;
            }
        }
        self.tail = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let old_head = unsafe { Box::from_raw(self.head) };
            self.head = old_head.next;

            if self.head.is_null() {
                self.tail = std::ptr::null_mut();
            }

            Some(old_head.elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}
