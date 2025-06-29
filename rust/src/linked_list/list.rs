pub type List = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
    pub val: i32,
    pub next: List,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: List) -> Self {
        ListNode { next, val }
    }

    pub fn iter(&self) -> ListIter {
        ListIter { cursor: Some(self) }
    }
}

impl FromIterator<i32> for ListNode {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut head = ListNode::default();
        let mut cursor = &mut head;
        for val in iter {
            cursor.next = Some(Box::new(ListNode::new(val, None)));
            cursor = cursor.next.as_mut().unwrap();
        }
        *head.next.unwrap()
    }
}

pub struct ListIter<'a> {
    cursor: Option<&'a ListNode>,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            Some(node) => {
                self.cursor = node.next.as_ref().map(|b| b.as_ref());
                Some(node.val)
            }
            None => None,
        }
    }
}
