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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = Box::new(ListNode::new(0));
        let mut tail = &mut new_list;

        let mut current = head;
        let mut curr_sum = 0;

        while let Some(node) = current {
            if node.val == 0 && curr_sum > 0 {
                tail.next = Some(Box::new(ListNode::new(curr_sum)));
                tail = tail.next.as_mut().unwrap();
                curr_sum = 0;
            } else {
                curr_sum += node.val;
            }
            current = node.next;
        }

        new_list.next
    }
}

fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    for val in values {
        tail.next = Some(Box::new(ListNode::new(val)));
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}

fn print_list(list: Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}

fn main() {
    let input = build_list(vec![1, 2, 0, 3, 4, 0, 5, 0]);
    let output = Solution::merge_nodes(input);
    print_list(output);
}
