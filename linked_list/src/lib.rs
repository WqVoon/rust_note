//! 学习一些链表的实现方式，学习自 <https://course.rs/too-many-lists/intro.html>

#[macro_use]
pub mod first;

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
    fn long_first() {
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
