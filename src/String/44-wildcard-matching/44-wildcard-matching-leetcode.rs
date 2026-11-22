
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut i = 0;
        let mut j = 0;

        let mut star: isize = -1;
        let mut match_index = 0;

        while i < s.len() {

            // Case 1: Characters match or '?' found
            if j < p.len() && (s[i] == p[j] || p[j] == b'?') {

                i += 1;
                j += 1;

            } else if j < p.len() && p[j] == b'*' {

                // Case 2: '*' found
                star = j as isize;
                match_index = i;
                j += 1;

            } else if star != -1 {

                // Case 3: Previous '*' exists
                j = star as usize + 1;
                match_index += 1;
                i = match_index;

            } else {

                // Case 4: No match possible
                return false;
            }
        }

        // Ignore remaining '*' characters
        while j < p.len() && p[j] == b'*' {
            j += 1;
        }

        j == p.len()
    }
}
