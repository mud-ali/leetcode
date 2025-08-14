impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        let mut i = 0;
        while i < words.len() {
            if words[i].contains(x) {
                ans.push(i as i32);
            }
            i+=1;
        }

        ans
    }
}