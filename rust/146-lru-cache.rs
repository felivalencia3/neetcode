/*
 * @lc app=leetcode id=146 lang=rust
 *
 * [146] LRU Cache
 */
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub struct ListNode {
    pub key: i32,
    pub value: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
    pub prev: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(key: i32, value: i32) -> Self {
        ListNode {
            key: key,
            value: value,
            next: None,
            prev: None,
        }
    }
}

struct LRUCache {
    capacity: i32,

    head: Rc<RefCell<ListNode>>,
    tail: Rc<RefCell<ListNode>>,

    map: HashMap<i32, Rc<RefCell<ListNode>>>,
}

impl LRUCache {
    fn insert(&mut self, node: Rc<RefCell<ListNode>>) {
        let mut _node = node.as_ref().borrow_mut();

        let mut _head = self.head.as_ref().borrow_mut();
        let _next_to_head = _head.next.as_ref();

        _node.next = _head.next.clone();

        if let Some(_next_to_head) = _next_to_head {
            _next_to_head.as_ref().borrow_mut().prev = Some(node.clone());
        }
        
        _head.next = Some(node.clone());
        _node.prev = Some(self.head.clone());

        self.map.insert(_node.key, node.clone());
    }

    fn remove(&mut self, node: Rc<RefCell<ListNode>>) {
        let mut _node = node.as_ref().borrow_mut();
        let mut _next_to_node = _node.next.as_ref();
        let mut _prev_to_node = _node.prev.as_ref();

        self.map.remove(&_node.key);

        if let Some(_next_to_node) = _next_to_node {
            _next_to_node.as_ref().borrow_mut().prev = _node.prev.clone();
        }

        if let Some(_prev_to_node) = _prev_to_node {
            _prev_to_node.as_ref().borrow_mut().next = _node.next.clone();
        }
    }

    fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(ListNode::new(0, 0)));
        let tail = Rc::new(RefCell::new(ListNode::new(0, 0)));

        head.as_ref().borrow_mut().next = Some(tail.clone());
        tail.as_ref().borrow_mut().prev = Some(head.clone());

        LRUCache {
            capacity,
            head: head,
            tail: tail,
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            let node_to_remove = node.clone();
            let node_to_insert = node.clone();
            self.remove(node_to_remove);
            self.insert(node_to_insert);

            let _node = &node.as_ref().borrow();

            return _node.value;
        } else {
            return -1;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            self.remove(node.clone());
        }

        if self.map.len() == self.capacity as usize {
            let _tail_clone = self.tail.clone();
            let _to_remove = _tail_clone.as_ref().borrow().prev.clone().unwrap();
            self.remove(_to_remove);
        }

        let _new_node = Rc::new(RefCell::new(ListNode::new(
            key,
            value
        )));

        self.insert(_new_node);
        
    }
}
// @lc code=end
