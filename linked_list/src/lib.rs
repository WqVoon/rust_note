//! 学习一些链表的实现方式，学习自 <https://course.rs/too-many-lists/intro.html>

#[macro_use]
pub mod first;
pub mod fourth;
pub mod second;
pub mod third;

#[cfg(test)]
mod test_first {
    use super::first;

    /// 测试 first 的基本功能
    #[test]
    fn basic() {
        let mut lst = first::List::<u32>::new();
        assert!(lst.pop() == None);

        lst.push(1);
        lst.push(2);
        lst.push(3);

        assert!(lst.pop() == Some(3));
        assert!(lst.pop() == Some(2));
        assert!(lst.pop() == Some(1));
        assert!(lst.pop() == None);
    }

    /// 测试长链表的 drop 是否会导致栈溢出
    #[test]
    fn long_list() {
        let mut lst = first::List::<u32>::new();
        for i in 0..100000 {
            lst.push(i);
        }
    }

    /// 测试打印效果
    #[test]
    fn display() {
        let mut lst = first::List::<u32>::new();
        assert!("[null]" == format!("{lst}"));

        lst.push(1);
        assert!("[1 -> null]" == format!("{lst}"));
        lst.push(2);
        assert!("[2 -> 1 -> null]" == format!("{lst}"));
        lst.push(3);
        assert!("[3 -> 2 -> 1 -> null]" == format!("{lst}"));

        lst.pop();
        assert!("[2 -> 1 -> null]" == format!("{lst}"));
        lst.pop();
        assert!("[1 -> null]" == format!("{lst}"));
        lst.pop();
        assert!("[null]" == format!("{lst}"));
        lst.pop();
        assert!("[null]" == format!("{lst}"));
    }
}

#[cfg(test)]
mod test_second {
    use super::second;

    /// 测试 second 的基本功能
    #[test]
    fn basic() {
        let mut lst = second::List::<u32>::new();
        assert!(lst.pop() == None);

        lst.push(1);
        lst.push(2);
        lst.push(3);

        assert!(lst.pop() == Some(3));
        assert!(lst.pop() == Some(2));
        assert!(lst.pop() == Some(1));
        assert!(lst.pop() == None);
    }

    /// 测试 second 的 peek
    #[test]
    fn peek() {
        let mut lst = second::List::<u32>::new();
        assert!(lst.peek() == None);

        lst.push(1);
        assert!(lst.peek() == Some(&1));
        assert!(lst.peek_mut() == Some(&mut 1));

        lst.peek_mut().map(|x| {
            *x += 1;
        });
        assert!(lst.peek() == Some(&2))
    }

    /// 测试 into_iter 方法
    #[test]
    fn into_iter() {
        let mut lst = second::List::<u32>::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut iter = lst.into_iter();
        assert!(iter.next() == Some(3));
        assert!(iter.next() == Some(2));
        assert!(iter.next() == Some(1));
        assert!(iter.next() == None);
    }

    /// 测试 iter 方法
    #[test]
    fn iter() {
        let mut lst = second::List::<u32>::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut iter = lst.iter();
        assert!(iter.next() == Some(&3));
        assert!(iter.next() == Some(&2));
        assert!(iter.next() == Some(&1));
        assert!(iter.next() == None);

        // lst 应该是可重复消费的
        let mut iter = lst.iter();
        assert!(iter.next() == Some(&3));
        assert!(iter.next() == Some(&2));
        assert!(iter.next() == Some(&1));
        assert!(iter.next() == None);
    }

    /// 测试 iter_mut 方法
    #[test]
    fn iter_mut() {
        let mut lst = second::List::<u32>::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut iter = lst.iter_mut();
        assert!(iter.next() == Some(&mut 3));
        assert!(iter.next() == Some(&mut 2));
        assert!(iter.next() == Some(&mut 1));
        assert!(iter.next() == None);

        // lst 应该是可重复消费的
        let mut iter = lst.iter_mut();
        assert!(iter.next() == Some(&mut 3));
        assert!(iter.next() == Some(&mut 2));
        assert!(iter.next() == Some(&mut 1));
        assert!(iter.next() == None);

        for v in lst.iter_mut() {
            *v *= 10;
        }
        assert!("[30 -> 20 -> 10 -> null]" == format!("{lst}"))
    }
}

#[cfg(test)]
mod test_third {
    use super::third;

    #[test]
    fn basic() {
        let list = third::List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    /// 测试长链表的 drop 是否会导致栈溢出
    #[test]
    fn long_list() {
        let mut lst = third::List::<u32>::new();
        for i in 0..100000 {
            lst = lst.prepend(i);
        }
    }
}

#[cfg(test)]
mod test_fourth {
    use super::fourth;

    #[test]
    fn basic() {
        let mut lst = fourth::List::<u32>::new();
        assert_eq!(lst.pop(), None);

        lst.push(1);
        lst.push(2);
        lst.push(3);
        assert_eq!(lst.peek(), Some(&1));
        assert_eq!(lst.pop(), Some(1));
        assert_eq!(lst.peek(), Some(&2));
        assert_eq!(lst.pop(), Some(2));
        assert_eq!(lst.peek(), Some(&3));
        assert_eq!(lst.pop(), Some(3));
        assert_eq!(lst.peek(), None);
        assert_eq!(lst.pop(), None);

        lst.push(1);
        *lst.peek_mut().unwrap() += 10;
        assert_eq!(lst.pop(), Some(11));
        assert_eq!(lst.pop(), None);
    }
}
