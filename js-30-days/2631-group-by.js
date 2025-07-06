/**
 * @param {Function} fn
 * @return {Object}
 */
Array.prototype.groupBy = function(fn) {
    let obj = {}
    for (const a of this) {
        const key = fn(a);
        if (Object.keys(obj).includes(key)) obj[key].push(a)
        else obj[key] = [a]
    }
    return obj
};

/**
 * [1,2,3].groupBy(String) // {"1":[1],"2":[2],"3":[3]}
 */