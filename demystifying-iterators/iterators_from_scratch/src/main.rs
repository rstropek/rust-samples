// This sample illustrates the implementation of iterators based on a simple linked list.
// Note that in practice, you SHOULD NOT implement such collections yourself, but rather use the
// collections provided by the standard library (e.g. Vec, LinkedList). We implement our
// own linked list here for educational purposes only.

/// Node for a simple single-linked list
///
/// The node contains a data field and a reference to the next node in the list.
struct Node<T> {
    /// The data stored in the node
    data: T,

    /// The next node in the list
    ///
    /// Option is used here to allow the next field to be set to None when the node is the last
    /// node in the list.
    ///
    /// Direct recursive types, where a struct directly contains another instance of itself,
    /// are not allowed in Rust because the size of the struct cannot be determined at compile
    /// time. A Box is a fixed size (a pointer size). The Box points to the next Node.
    /// From the perspective of the Rust compiler, the size of Node is known and fixed,
    /// making the recursive data structure feasible.
    next: Option<Box<Node<T>>>,
}

/// A simple single-linked list
struct LinkedList<T> {
    /// The head of the list
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Creates a new LinkedList
    fn new() -> Self {
        LinkedList { head: None }
    }

    /// Adds a new element at the end of the list
    ///
    /// This method takes ownership of the data and adds it to the end of the list.
    /// 
    /// ## Example
    /// 
    /// ```
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// ```
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        // Traverse the list to find the last node. A mutable reference to the head
        // field of the LinkedList is obtained and stored in the variable current.
        // This variable will be used to traverse the list.
        let mut current = &mut self.head;

        // The loop continues as long as the current node (starting with the head) is not None.
        // This is checked using the while let Some(...) = *current construct, which also
        // allows modifying the next field of the nodes during traversal.
        while let Some(ref mut next_node) = *current {
            // current is updated to point to the next field of the current node. This
            // effectively moves the traversal pointer one node forward in the list.
            current = &mut next_node.next;
        }

        *current = Some(new_node);
    }

    fn iter(&self) -> LinkedListRefIterator<T> {
        LinkedListRefIterator {
            current: self.head.as_deref(),
        }
    }
}

/// Makro to create a linked list from multiple elements (similar to vec![])
/// 
/// ## Example
/// 
/// ```
/// let list = linked_list![1, 2, 3];
/// ```
macro_rules! linked_list {
    ($($e:expr),*) => {{
        let mut list = LinkedList::new();
        $(list.push($e);)*
        list
    }};
}

/// Implementing IntoIterator for LinkedList allows the use of the for loop syntax to iterate over
///
/// Implements a consuming iterator for LinkedList. The iterator takes ownership of the list and
/// returns the elements by value.
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        // The head field of the LinkedList is moved into the LinkedListIterator.
        LinkedListIterator::new(self.head)
    }
}

/// Iterator for LinkedList
struct LinkedListIterator<T> {
    current: Option<Box<Node<T>>>,
}

impl<T> LinkedListIterator<T> {
    /// Creates a new LinkedListIterator
    fn new(head: Option<Box<Node<T>>>) -> Self {
        LinkedListIterator { current: head }
    }
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // The current field is taken out of the LinkedListIterator and replaced with None.
        // This is done using the Option::take method, which replaces the value of the Option
        // with None and returns the original value.
        // The node is moved out of the Box and the next field is assigned to the current field.
        // This is done using the Option::map method, which applies a closure to the value of the
        // Option if it is Some, and returns the result as a new Option.
        self.current.take().map(|node| {
            // The current field is updated to point to the next field of the node. This effectively
            // moves the iterator one step forward in the list.
            self.current = node.next;

            // The data field of the node is returned by value.
            node.data
        })
    }
}

/// Implementing IntoIterator for &LinkedList allows the use of the for loop syntax to iterate over
///
/// Implements a non-consuming iterator for LinkedList. The iterator borrows the list and returns
/// the elements by reference.
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListRefIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // The head field of the LinkedList is borrowed and passed to the LinkedListRefIterator.
        LinkedListRefIterator::new(&self.head)
    }
}

/// Iterator for &LinkedList
struct LinkedListRefIterator<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> LinkedListRefIterator<'a, T> {
    /// Creates a new LinkedListRefIterator
    fn new(head: &'a Option<Box<Node<T>>>) -> Self {
            // The head field of the LinkedList is borrowed and passed to the LinkedListRefIterator.
            // The map method is used to dereference the node twice to get a reference to the data field.
            LinkedListRefIterator {
            current: head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for LinkedListRefIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            // The current field is updated to point to the next field of the node. This effectively
            // moves the iterator one step forward in the list.
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

/// Implementing IntoIterator for &mut LinkedList allows the use of the for loop syntax to iterate over
/// 
/// Implements a mutable iterator for LinkedList. The iterator borrows the list mutably and returns
/// the elements by mutable reference.
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = LinkedListMutRefIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // The head field of the LinkedList is borrowed mutably and passed to the LinkedListMutRefIterator.
        LinkedListMutRefIterator::new(&mut self.head)
    }
}

/// Mutable iterator for &mut LinkedList
struct LinkedListMutRefIterator<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> LinkedListMutRefIterator<'a, T> {
    /// Creates a new LinkedListMutRefIterator
    fn new(head: &'a mut Option<Box<Node<T>>>) -> Self {
            // The head field of the LinkedList is borrowed mutably and passed to the LinkedListMutRefIterator.
            // The map method is used to dereference the node twice to get a mutable reference to the data field.
            LinkedListMutRefIterator {
            current: head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for LinkedListMutRefIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            // The current field is updated to point to the next field of the node. This effectively
            // moves the iterator one step forward in the list.
            self.current = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

fn main() {
    let list = linked_list![1, 2, 3];
    let mut iterator = list.iter();
    println!("{}", iterator.next().unwrap());
    
    let mut list = linked_list![1, 2, 3];
    let _iter = (&list).into_iter();
    let _iter = (&mut list).into_iter();
    let _iter = list.into_iter();

    let mut list = linked_list![1, 2, 3];
    
    for i in &list {
        println!("{}", i);
    }
    
    for i in &mut list {
        *i += 1;
    }
    
    for i in list {
        println!("{}", i);
    }
}
