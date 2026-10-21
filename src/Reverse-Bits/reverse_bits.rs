impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let binary_str = format!("{:032b}", n);
        let reversed_str: String = binary_str.chars().rev().collect();
        let reversed_decimal = u32::from_str_radix(&reversed_str, 2).unwrap();
        return reversed_decimal as i32;



    }
}
