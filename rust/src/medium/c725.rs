// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution;
impl Solution {
  fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut result: Vec<Option<Box<ListNode>>> = Vec::new();
    let mut head = head.clone();
    let len = if let Some(h) = head.clone() {
      Solution::len(&h)
    } else {
      0
    };

    let ave = len / k;
    let mut rem = len % k;

    let mut pre: Option<Box<ListNode>> = None;
    for _ in 0..k {
      let header = head.clone();
      result.push(header);
      let temp = if rem > 0 { ave + 1 } else { ave };
      for _ in 0..temp {
        let mut headle = head.take();
        match pre.take() {
          Some(node) => {
            match headle.take() {
              Some(n) => Solution::push(node, n.val),
              None => continue,
            }
          },
          None => pre = headle,
        }
      }

      if pre != None {
        match pre.take() {
          Some(mut node) => node.next = None,
          None => continue,
        }
      }

      if rem > 0 {
        rem -= 1;
      }
    }

    result
  }

  fn len(head: &Box<ListNode>) -> i32 {
    let mut node = head.clone();
    match node.next.take() {
      Some(tail) => 1 + Solution::len(&tail),
      None => 1,
    }
  }

  fn push(mut node: Box<ListNode>, val: i32) {
    if let Some(node) = node.next {
      Solution::push(node, val);
    } else {
      node.next = Some(Box::new(ListNode { val, next: None }));
    }
  }
}

pub fn test(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
  Solution::split_list_to_parts(head, k)
}
