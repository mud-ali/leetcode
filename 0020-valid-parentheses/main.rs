impl Solution {

    fn get_open(c : char) -> char {
        match c {
            ')'=>'(',
            '}'=>'{',
            ']'=>'[',
            _ => '/'
        }
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for e in s.chars() {
            let o = Self::get_open(e);
            if o=='/' {
                stack.push(e);
            } else if stack.len() == 0 || o!=stack[stack.len()-1] {
                return false;
            } else {
                stack.pop();
            }
        }
        return true;
    }
}