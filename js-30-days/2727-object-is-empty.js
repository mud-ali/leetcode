/**
 * @param {Object|Array} obj
 * @return {boolean}
 */
var isEmpty = function(obj) {
    if (typeof obj == 'object' && Array.from(Object.keys(obj)).length === 0) return true
    return typeof obj !== 'object' && obj.length === 0
};