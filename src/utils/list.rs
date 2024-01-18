#[derive(Debug, PartialEq)]
struct ListNodeValue<T> {
    item: T,
    next: Box<ListNode<T>>,
}

impl<T> ListNodeValue<T> {
    fn new(item: T, next: Box<ListNode<T>>) -> Self {
        Self { item, next }
    }
}

impl<T> Clone for ListNodeValue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            next: Box::clone(&self.next),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ListNode<T> {
    Empty,
    NonEmpty(ListNodeValue<T>),
}

impl<T> Default for ListNode<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> ListNode<T> {
    fn new(item: T, next: Box<ListNode<T>>) -> Self {
        Self::NonEmpty(ListNodeValue::new(item, next))
    }

    fn take(&mut self) -> Self {
        let mut cur = Self::Empty;
        std::mem::swap(&mut cur, self);
        cur
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SinglyLinkedList<T> {
    head: Box<ListNode<T>>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Box::new(ListNode::Empty),
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        let cur_head = self.head.take();
        let new_node = Box::new(ListNode::new(item, Box::new(cur_head)));

        self.head = new_node;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take();

        if let ListNode::NonEmpty(node) = node {
            self.head = node.next;
            self.size -= 1;
            Some(node.item)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> Iterator for SinglyLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[macro_export]
macro_rules! slist {
    () => {SinglyLinkedList::new()};
    ($($element:expr,)*) => {{
        let mut list = SinglyLinkedList::new();
        $(
            {
                list.push($element);
            }
        )*
        list
    }};
    ($($element:expr),*) => {{
        slist!($($element,)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn it_works() {
        let mut linked_list: SinglyLinkedList<usize> = SinglyLinkedList::new();
        for i in 1..=10 {
            linked_list.push(i);
        }

        for i in (1..=10).rev() {
            let cur = linked_list.pop();
            assert_eq!(Some(i), cur);
        }

        assert_eq!(None, linked_list.pop());
    }

    #[test]
    fn test_series_of_pops_and_inserts() {
        let mut list: SinglyLinkedList<usize> = SinglyLinkedList::new();
        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(42);
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.len(), 1);

        list.push(93);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(93));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        list.push(20);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_macro_empty() {
        let mut list: SinglyLinkedList<usize> = slist![];
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_macro_one() {
        let mut list: SinglyLinkedList<usize> = slist![42];
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_macro_two() {
        let mut list: SinglyLinkedList<usize> = slist![42, 50];
        assert_eq!(list.pop(), Some(50));
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_macro_with_comma() {
        let mut list: SinglyLinkedList<usize> = slist![
            42, 50, 1, 10, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26, 27, 28, 29, 30,
        ];
        let mut vector = vec![
            42, 50, 1, 10, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26, 27, 28, 29, 30,
        ];

        assert_eq!(list.len(), vector.len());
        while let Some(value) = list.pop() {
            assert_eq!(Some(value), vector.pop());
        }

        assert_eq!(list.pop(), None);
        assert_eq!(vector.pop(), None);
    }

    #[test]
    fn test_iter() {
        let list = slist![1, 2, 3, 4, 5];
        let vector: Vec<_> = list.collect();

        assert_eq!(vector, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_clone() {
        let list = slist![1, 2, 3, 4, 5];
        let cloned = list.clone();

        assert_eq!(list.len(), 5);
        assert_eq!(list, cloned);
    }
}
