//! 用 `Option<Box>` 实现的单向链表，支持 push, pop, peak, iterator

use std::fmt::Display;

/// 链表的主体
pub struct List<T> {
    ptr: Link<T>,
}

/// 类型别名
type Link<T> = Option<Box<Node<T>>>;

/// 链表节点，保存具体数据
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
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

    /// `pop` 从链表中弹出一个值，
    /// 原本的 match Option 可以换成 Option.map 方法调用
    pub fn pop(&mut self) -> Option<T> {
        self.ptr.take().map(|node| {
            self.ptr = node.next;
            node.data
        })
    }

    /// `peak` 返回链表头部的值的不可变引用
    pub fn peek(&self) -> Option<&T> {
        self.ptr.as_ref().map(|node| &node.data)
    }

    /// `peek_mut` 返回链表头部的值的可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.ptr.as_mut().map(|node| &mut node.data)
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

/// List.into_iter 的产物，实现了 Iterator Trait
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// List.iter 的产物，实现了 Iter
pub struct Iter<'a, T> {
    ptr: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            ptr: self.ptr.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|node| {
            self.ptr = node.next.as_deref();
            &node.data
        })
    }
}

/// List.iter_mut 的产物，实现了 IterMut
pub struct IterMut<'a, T> {
    ptr: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            ptr: self.ptr.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|node| {
            self.ptr = node.next.as_deref_mut();
            &mut node.data
        })
    }
}
