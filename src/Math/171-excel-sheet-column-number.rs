
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i32 = 0;

        for char in column_title.chars() {
            result = result * 26;

            let letter_value: i32 = (char as i32) - ('A' as i32) + 1;

            result = result + letter_value;
        }

        return result;
    }
}
