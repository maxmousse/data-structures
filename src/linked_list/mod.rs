/// Immutable linked list (https://rust-unofficial.github.io/too-many-lists/third.html)
///
/// It user Rc to store data of the heap, allow recursive type and keep track to several
/// owners at the same time (the way the linked list work in functional languages)
///
/// Note: as it uses Rc, this list is not thread safe. It could be updated to use Arc instead
/// (https://rust-unofficial.github.io/too-many-lists/third-arc.html)
pub mod immutable_linked_list;

/// Mutable linked list (https://rust-unofficial.github.io/too-many-lists/second.html)
///
/// It uses Box to store data on the heap and allow a recursive type
/// Linked node are mutable and owned by the list (only on owner)
pub mod mutable_linked_list;

pub mod doubly_linked_list;
