/**
 * @param {Function} fn
 * @return {Function}
 */
function memoize(fn) {
    const cache = {}
    return function(...args) {
        let argString = args.join(",");
        if (cache[argString]!== undefined) {
            return cache[argString]
        }
        cache[argString] = fn(...args)
        return cache[argString]
    }
}


/** 
 * let callCount = 0;
 * const memoizedFn = memoize(function (a, b) {
 *	 callCount += 1;
 *   return a + b;
 * })
 * memoizedFn(2, 3) // 5
 * memoizedFn(2, 3) // 5
 * console.log(callCount) // 1 
 */