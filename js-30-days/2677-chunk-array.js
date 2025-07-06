/**
 * @param {Array} arr
 * @param {number} size
 * @return {Array}
 */
var chunk = function(arr, size) {
    const ret = []
    let current = []
    for (const item of arr)
    {
        if (current.length === size) {
            ret.push(current)
            current = []
        }
        current.push(item)
    }
    if (current.length !== 0) ret.push(current)
    return ret
};
