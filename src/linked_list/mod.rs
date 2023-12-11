use core::fmt;
use std::{fmt::Display, usize};

#[derive(Copy, PartialEq, Debug)]
pub struct Node<T>
where
    T: Copy + Display,
{
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Copy + Display,
{
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }

    pub fn append(&mut self, node: Box<Node<T>>) {
        self.next = Some(node);
    }
}

#[derive(Copy, PartialEq, Debug)]
pub struct SinglyLinkedList<T>
where
    T: Copy + Display,
{
    length: usize,
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T> fmt::Display for SinglyLinkedList<T>
where
    T: Copy + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        write!(f, "LinkedList: [")?;

        while let Some(node) = current {
            write!(f, "{}", node.value)?;

            if node.next.is_some() {
                write!(f, ", ")?;
            }

            current = &node.next;
        }

        write!(f, "]")
    }
}

impl<T> SinglyLinkedList<T>
where
    T: Copy + Display,
{
    pub fn new() -> Self {
        SinglyLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        match self.tail.take() {
            Some(mut old_tail) => {
                old_tail.next = Some(new_node);
                self.tail = Some(old_tail.next.unwrap());
            }
            None => {
                self.head = Some(new_node);
                self.tail = Some(self.head.as_mut().unwrap());
            }
        }

        // match self.is_empty() {
        //     true => {
        //         self.head = Some(new_node);
        //         self.tail = Some(self.head.as_mut().unwrap().clone()); // Not efficient to clone, but else require unsafe, I think
        //     }
        //     false => {
        //         let mut old_tail = self.tail.take().unwrap();
        //         old_tail.next = Some(new_node);
        //         self.tail = old_tail.next;
        //     }
        // }

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // Get before last node
        match self.get(self.length - 2) {
            Some(before_last_node) => {
                // Extract last node value
                let last_node_value = before_last_node.next.as_ref().unwrap().value;
                // Update tail
                self.tail = Some(Box::new(before_last_node));
                self.tail.as_mut().unwrap().next = None;

                Some(last_node_value)
            }
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn get(&mut self, index: usize) -> Option<&mut Node<T>> {
        if index >= self.length {
            return None;
        }

        let mut current = &mut self.head;

        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return None; // Unexpected case: the list is shorter than expected
            }
        }

        current.as_deref_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_new() {
        assert_eq!(
            Node::new(5),
            Node {
                value: 5,
                next: None
            }
        );
    }

    #[test]
    fn node_append() {
        let mut node_1 = Node::new(1);

        node_1.append(Box::new(Node::new(2)));

        assert_eq!(node_1.next.unwrap(), Box::new(Node::new(2)));
    }

    #[test]
    fn list_is_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();

        assert_eq!(list.is_empty(), true);

        list.push(1);

        assert_eq!(list.is_empty(), false);
    }

    #[test]
    fn list_push() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        println!("List 1: {}", list);

        list.push(1);
        println!("List 2: {}", list);

        assert_eq!(*list.head.as_ref().unwrap(), Box::new(Node::new(1)));
        assert_eq!(*list.tail.as_ref().unwrap(), Box::new(Node::new(1)));
        assert_eq!(list.length, 1);

        list.push(2);
        println!("List 3: {}", list);

        assert_eq!(
            *list.head.as_ref().unwrap().next.as_ref().unwrap(),
            Box::new(Node::new(2))
        );
        assert_eq!(*list.head.as_ref().unwrap(), Box::new(Node::new(1)));
        assert_eq!(*list.tail.as_ref().unwrap(), Box::new(Node::new(2)));
        assert_eq!(list.length, 2);
    }

    #[test]
    fn list_get() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();

        assert_eq!(list.get(0), None);

        list.push(1);
        list.push(2);

        assert_eq!(*list.get(0).unwrap(), Node::new(1));
        assert_eq!(list.get(2), None);
        assert_eq!(*list.get(1).unwrap(), Node::new(2));
    }

    #[test]
    fn list_pop() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(*list.head.as_ref().unwrap(), Box::new(Node::new(1)));
        assert_eq!(*list.tail.as_ref().unwrap(), Box::new(Node::new(3)));
        assert_eq!(list.length, 3);

        let mut popped = list.pop();

        assert_eq!(popped.unwrap(), 3);
        assert_eq!(*list.tail.as_ref().unwrap(), Box::new(Node::new(2)));
        assert_eq!(list.length, 2);

        popped = list.pop();

        assert_eq!(popped.unwrap(), 2);
        assert_eq!(*list.tail.as_ref().unwrap(), Box::new(Node::new(1)));
        assert_eq!(list.length, 1);

        popped = list.pop();

        assert_eq!(popped.unwrap(), 1);

        assert_eq!(list.head.as_ref(), None);
        assert_eq!(list.tail.as_ref(), None);
        assert_eq!(list.length, 0);

        assert_eq!(popped, None);

        assert_eq!(list.head.as_ref(), None);
        assert_eq!(list.tail.as_ref(), None);
        assert_eq!(list.length, 0);
    }
}
