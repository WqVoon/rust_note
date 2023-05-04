//! 通过 `Rc` 实现的单向不可变链表，支持 prepend，tail，head
use std::rc::Rc;

pub struct List<T> {
    ptr: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { ptr: None }
    }

    /// `prepend` 在链表前增加一个节点
    pub fn prepend(&self, data: T) -> List<T> {
        List {
            ptr: Some(Rc::new(Node {
                data,
                // 直接调用 clone 等价于下面的调用方式：
                //  next: self.ptr.as_ref().map(|node| Rc::clone(node)),
                next: self.ptr.clone(),
            })),
        }
    }

    /// `tail` 返回入参去掉头节点后的链表
    pub fn tail(&self) -> List<T> {
        List {
            ptr: self.ptr.as_ref().and_then(|node| node.next.clone()),
        }
    }

    /// `head` 返回链表头节点中的内容的不可变引用
    pub fn head(&self) -> Option<&T> {
        self.ptr.as_ref().map(|node| &node.data)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut scanner = self.ptr.take();
        while let Some(rc_node) = scanner {
            if let Ok(mut node) = Rc::try_unwrap(rc_node) {
                scanner = node.next.take();
            } else {
                break;
            }
        }
    }
}
