use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;

const MAX_ITER: i32 = 300000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // TODO: your code here, for linked list insertions
    linked_list_operations();

    // TODO: your code here, for hashmap insertions
    hashmap_operatins();

    // TODO: your text explanation to the questions in the spec
    // Which collection type was the fastest for adding and removing elements? Why do you think this was the case?
    //     - For insertion, Vec was the fastest collection. This is as inserting to the end of a vector does not require any shifts and is achieved in O(1) time. Additionally, array-based containers are faster, more memory efficient and make better use of CPU cache making Vec and Vecdeque superior to linked lists.
    //     - For deletion, Vec was also the fastest which is surprising as VecDeque should theoretically outperform Vec when removing elements from the start of a vector. I suspect this to be because when there is a large number of elements, memory is reallocated and this requires shifting of vector elements depending on position of head and tail which has some overhead costs. 

    // Is there any significant difference between Vec and VecDeque deletion?
    //     Vec is slightly faster by 0.7seconds 

    // When would you consider using VecDeque over Vec?
    //     Theoretically, VecDeque is more efficient for insertion and deletion at front of vector as it is implemented using a head and a tail pointer.
    //     Good use cases would be implementation of a queue by using push_back to add to the queue and pop_front to remove from the front of queue. 

    // When would you consider using LinkedList over Vec?
    //     When pushing and popping elements at either end as it can be achieved in constant time using a doubly linked list. Removing elements from start of Vec requires O(n) shifts. 

    // Did the results suprise you? Why or why not?.
    //     - Linked lists was significantly slower than Vec operations. I found that this was due to time it takes to construct and maintain a pointer for each node.
    //     - Vec did better than VecDeque for popping elements from front which seems to contradict rust documentation. 

}

/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn linked_list_operations() {
    let mut ll = LinkedList::new();
    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        ll.push_back(i);
    }
    let time_end = std::time::Instant::now();
    println!("==== Linked List ====");
    println!("insert: {:?}", time_end - time_start);
    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        ll.pop_front();
    }
    let time_end = std::time::Instant::now();
    println!("remove: {:?}", time_end - time_start);
}

fn hashmap_operatins() {
    let mut hm = HashMap::new();
    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hm.insert(i, i);
    }
    let time_end = std::time::Instant::now();
    println!("==== Hashmap ====");
    println!("insert: {:?}", time_end - time_start);
    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hm.remove(&i);
    }
    let time_end = std::time::Instant::now();
    println!("remove: {:?}", time_end - time_start);
}