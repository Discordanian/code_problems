#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for entry in vec.iter().rev() {
            let mut node = Self::new(*entry);
            node.next = result;
            result = Some(Box::new(node));
        }
        result
    }

    /*
    pub fn to_vec(&self) -> Vec<i32> {
        let mut retval = Vec::new();

        let mut list = *self.unwrap();
        while let Some(entry) = list {
            retval.push(entry.val);
            list = list.next;
        }
        return retval;
    }
    */
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

    /*
    #[test]
    fn list_to_vec() {
        let input = vec![1];
        let mut list = ListNode::from_vec(&input);
        assert_eq!(input, list.to_vec());
    }
    */
}
