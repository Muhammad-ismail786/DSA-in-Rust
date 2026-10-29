impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut map_s = [0usize; 256];
        let mut map_t = [0usize; 256];

        for i in 0..s.len() {
            if map_s[s[i] as usize] != map_t[t[i] as usize] {
                return false;
            }

            map_s[s[i] as usize] = i + 1;
            map_t[t[i] as usize] = i + 1;
        }

        true
    }
}
