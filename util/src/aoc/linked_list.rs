use std::mem;
use std::ops::{Index, IndexMut};

/// A doubly-linked list that lets you actually do O(1) insertion.
pub struct UsefulLinkedList<T> {
    // There's probably a nice slab allocator out there.
    nodes: Vec<Option<Node<T>>>,
    // But for now we'll write our own with our own free list.
    free: Vec<Idx>,
    head: Option<Idx>,
    tail: Option<Idx>,
}

impl<T> UsefulLinkedList<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> UsefulLinkedList<T> {
        UsefulLinkedList {
            nodes: Vec::new(),
            free: Vec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn head(&self) -> Option<Idx> {
        self.head
    }

    pub fn tail(&self) -> Option<Idx> {
        self.tail
    }

    pub fn next(&self, node_index: Idx) -> Option<Idx> {
        self[node_index].next
    }

    pub fn prev(&self, node_index: Idx) -> Option<Idx> {
        self[node_index].prev
    }

    pub fn push_back(&mut self, item: T) -> Idx {
        match self.tail {
            Some(t) => self.insert_after(t, item),
            None => {
                let node = Node {
                    value: item,
                    next: None,
                    prev: None,
                };
                let idx = self.next_node_idx();
                self.nodes[idx.0] = Some(node);
                self.tail = Some(idx);
                if self.head.is_none() {
                    self.head = Some(idx);
                }

                idx
            }
        }
    }

    pub fn insert_after(&mut self, node_index: Idx, item: T) -> Idx {
        let new_node_idx = self.next_node_idx();

        let new_node = Node {
            value: item,
            next: self[node_index].next,
            prev: Some(node_index),
        };
        self.nodes[new_node_idx.0] = Some(new_node);

        if let Some(next) = self[node_index].next {
            self[next].prev = Some(new_node_idx);
        }

        self[node_index].next = Some(new_node_idx);

        if self.tail == Some(node_index) {
            self.tail = Some(new_node_idx)
        }

        new_node_idx
    }

    pub fn remove(&mut self, node_index: Idx) -> T {
        let mut removed_node: Option<Node<T>> = None;
        mem::swap(&mut removed_node, &mut self.nodes[node_index.0]);
        self.free.push(node_index);

        let removed_node = removed_node.unwrap();

        if Some(node_index) == self.head {
            self.head = removed_node.next
        }

        if Some(node_index) == self.tail {
            self.tail = removed_node.prev
        }

        if let Some(prev) = removed_node.prev {
            self[prev].next = removed_node.next;
        }
        if let Some(next) = removed_node.next {
            self[next].prev = removed_node.prev;
        }

        removed_node.value
    }

    fn next_node_idx(&mut self) -> Idx {
        if self.free.is_empty() {
            let new_index = Idx(self.nodes.len());
            self.nodes.push(None);
            new_index
        } else {
            self.free.pop().unwrap()
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for UsefulLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        let mut n = self.head;

        write!(f, "List(")?;
        while let Some(node) = n {
            if !first {
                write!(f, ", ")?;
            }
            first = false;

            write!(f, "{:?}", self[node].value)?;
            n = self[node].next;
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl<T> Index<Idx> for UsefulLinkedList<T> {
    type Output = Node<T>;

    fn index(&self, index: Idx) -> &Node<T> {
        match self.nodes[index.0] {
            None => panic!("Invalid index."),
            Some(ref node) => node,
        }
    }
}

impl<T> IndexMut<Idx> for UsefulLinkedList<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Node<T> {
        match self.nodes[index.0] {
            None => panic!("Invalid index."),
            Some(ref mut node) => node,
        }
    }
}
//
//impl<T> IntoIterator for UsefulLinkedList<T> {
//    type Item = T;
//    type IntoIter = UsefulLinkedListIterator<T>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        UsefulLinkedListIterator {
//            index: self.head.clone(),
//            linked_list: self,
//        }
//    }
//}
//
//struct UsefulLinkedListIterator<T> {
//    linked_list: UsefulLinkedList<T>,
//    index: Option<Idx>
//}
//
//impl<T> Iterator for UsefulLinkedListIterator<T> {
//    type Item = T;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        match self.index {
//            Some(idx) => {
//                let result = &self.linked_list[idx];
//                self.index = result.next;
//                Some(result.value)
//            },
//            None => None,
//        }
//    }
//}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Idx(usize);

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Idx>,
    pub prev: Option<Idx>,
}
//
//mod test {
//    use super::*;
//
//    #[test]
//    fn insertion() {
//        let mut ll = UsefulLinkedList::new();
//
//        ll.push_back(1);
//        let two = ll.push_back(2);
//        ll.push_back(4);
//        ll.insert_after(two, 3);
//
//        let result: Vec<i32> = ll.into_iter().collect();
//
//        assert_eq!(vec![1,2,3,4], result);
//    }
//}
