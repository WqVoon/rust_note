//! 用 `Option<Box>` 实现的单向链表，支持 push 和 pop

use std::fmt::Display;

/// 链表的主体
pub struct List<T> {
    ptr: Option<Box<Node<T>>>,
}

/// 链表节点，保存具体数据
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Display + Copy> List<T> {
    /// `new` 创建一个链表实例
    pub fn new() -> Self {
        List { ptr: None }
    }

    /// `push` 向链表中压入一个值
    pub fn push(&mut self, data: T) {
        self.ptr = Some(Box::new(Node {
            data,
            next: self.ptr.take(),
        }))
    }

    /// `pop` 从链表中弹出一个值
    pub fn pop(&mut self) -> Option<T> {
        match self.ptr.take() {
            None => None,
            Some(node) => {
                self.ptr = node.next;
                Some(node.data)
            }
        }
    }
}

impl<T> Drop for List<T> {
    /// 默认的 drop 会因为递归而导致栈溢出，所以需要自己实现 drop
    fn drop(&mut self) {
        let mut scanner = self.ptr.take();
        while let Some(mut t) = scanner {
            scanner = t.next.take()
        }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut scanner = &self.ptr;
        while let Some(t) = scanner {
            write!(f, "{} -> ", t.data)?;
            scanner = &t.next;
        }

        write!(f, "null]")
    }
}
