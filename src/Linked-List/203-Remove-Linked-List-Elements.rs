
impl Solution {
    pub fn remove_elements(
        mut head: Option<Box<ListNode>>,
        val: i32,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while current.is_some() {
            if current.as_ref().unwrap().val == val {
                *current = current.take().unwrap().next;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }

        head
    }
}
