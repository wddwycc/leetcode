use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn move_to_head(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match (&prev, &next) {
            (None, None) => {
                // this is the only node in list, so do nothng
            }
            (Some(_), None) => {
                // this node is already the head, so do nothing
            }
            (None, Some(next)) => {
                // this is the tail, move it to front
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                next.borrow_mut().prev = None;
                self.tail = Some(Rc::clone(next));

                let prev_head = self.head.as_ref().unwrap();
                prev_head.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_head));
                self.head = Some(Rc::clone(node));
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;

                prev.borrow_mut().next = Some(Rc::clone(next));
                next.borrow_mut().prev = Some(Rc::clone(prev));

                let prev_head = self.head.as_ref().unwrap();
                prev_head.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_head));
                self.head = Some(Rc::clone(node));
            }
        }
    }

    fn push_front(&mut self, node: &Rc<RefCell<Node>>) {
        match &self.head {
            None => {
                self.head = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            }
            Some(prev_head) => {
                Rc::clone(node).borrow_mut().prev = Some(Rc::clone(prev_head));
                prev_head.borrow_mut().next = Some(Rc::clone(node));
                self.head = Some(Rc::clone(node));
            }
        }
    }

    fn remove_tail(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(tail) = self.tail.as_ref().map(|a| a.clone()) {
            if let Some(tail_next) = tail.borrow().next.as_ref() {
                tail_next.borrow_mut().prev = None;
                self.tail = Some(Rc::clone(tail_next));
            }
            tail.borrow_mut().next = None;
            tail.borrow_mut().prev = None;
            return Some(tail);
        }
        None
    }
}

struct LRUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    list: LinkedList,
    size: i32,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            list: LinkedList::new(),
            size: 0,
            capacity: capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            self.list.move_to_head(node);
            return node.borrow().value;
        } else {
            return -1;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            self.list.move_to_head(node);
            node.borrow_mut().value = value;
        } else {
            // remove tail node if needed
            if self.size >= self.capacity {
                if let Some(prev_tail) = self.list.remove_tail() {
                    self.map.remove(&prev_tail.borrow().key);
                };
            }
            // add node to list head
            let node = Rc::new(RefCell::new(Node {
                prev: None,
                next: None,
                key: key,
                value: value,
            }));
            self.list.push_front(&node);
            // update hashmap
            self.map.insert(key, node);
            // update size
            self.size += 1;
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

/**
    LRUCache cache = new LRUCache( 2 /* capacity */ );

    cache.put(1, 1);
    cache.put(2, 2);
    cache.get(1);       // returns 1
    cache.put(3, 3);    // evicts key 2
    cache.get(2);       // returns -1 (not found)
    cache.put(4, 4);    // evicts key 1
    cache.get(1);       // returns -1 (not found)
    cache.get(3);       // returns 3
    cache.get(4);       // returns 4
*/
fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
