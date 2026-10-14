impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut result = String::new();

        while column_number > 0 {
            column_number -= 1;

            let remainder = column_number % 26;

            let letter = (b'A' + remainder as u8) as char;

            result.push(letter);

            column_number /= 26;
        }

        let reversed: String = result.chars().rev().collect();

        reversed
    }
}
