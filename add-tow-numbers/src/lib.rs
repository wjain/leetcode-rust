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

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy1 = l1;
        let mut dummy2 = l2;

        let mut carry = 0;

        let mut root = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut root;

        while dummy1.is_some() || dummy1.is_some() {
            match curr {
                Some(inner_node) => {
                    let first = dummy1.take().unwrap_or(Box::new(ListNode::new(0)));
                    let second = dummy2.take().unwrap_or(Box::new(ListNode::new(0)));

                    let mut sum = first.val + second.val + carry;

                    carry = sum / 10;
                    sum = sum % 10;

                    inner_node.next.get_or_insert(Box::new(ListNode::new(sum)));

                    curr = &mut inner_node.next;
                    dummy1 = first.next;
                    dummy2 = second.next;
                }
                None => break,
            }
        }

        if carry == 1 {
            if let Some(node) = curr {
                node.next.get_or_insert(Box::new(ListNode::new(1)));
            }
        }

        root.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        assert_eq!(
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
            Solution::add_two_numbers(l1, l2)
        );
    }
}
