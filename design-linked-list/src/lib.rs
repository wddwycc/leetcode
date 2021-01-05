struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    /// O(n)
    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            Some(ref a) => a,
            None => return -1,
        };
        let mut idx_cur = 0;
        while idx_cur < index {
            if let Some(ref next) = cur.next {
                cur = next;
                idx_cur += 1;
            } else {
                return -1;
            };
        }
        cur.val
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    /// O(1)
    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }))
    }

    /** Append a node of value val to the last element of the linked list. */
    /// O(n)
    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut a) => a,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };
        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(Box::new(Node { val, next: None }));
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    /// O(n)
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = Some(Box::new(Node {
            val,
            next: cur.next.take(),
        }));
        self.head = dummy_head.next;
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    /// O(n)
    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
