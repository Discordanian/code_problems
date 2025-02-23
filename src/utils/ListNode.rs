#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
    fn from_vec(vector: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &value in vector.iter().rev() {
            let mut new_node = ListNode::new(value);
            new_node.next = head;
            head = Some(Box::new(new_node));
        }
        head
    }
    fn to_vec(&self) -> Vec<i32> {
        let mut retval = Vec::new();
        retval.push(self.val);
        let mut ptr = &self.next;
        while let Some(l) = ptr {
            retval.push(l.val);
            ptr = &l.next;
        }
        retval
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_list() {
        let mut list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);

        let mut n = 1;
        while let Some(entry) = list {
            assert_eq!(entry.val, n);
            n += 1;
            list = entry.next;
        }
    }
}
