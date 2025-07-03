function mergeAlternately(word1: string, word2: string): string {
    let output = "";
    let i1 = 0, i2 = 0;
    for (let i = 0; i < word1.length + word2.length; i++) {
        if (i % 2 === 0) {
            if (i1 < word1.length) {
                output += word1.charAt(i1++);
                continue;
            }
        } else if (i2 < word2.length) {
            output += word2.charAt(i2++);
            continue;
        }
        if (i1 >= word1.length) {
            output += word2.substring(i2);
            break;
        }
        if (i2 >= word2.length) {
            output += word1.substring(i1);
            break;
        }
    }
    return output;
}

console.log(mergeAlternately("abc", "pqr"));
