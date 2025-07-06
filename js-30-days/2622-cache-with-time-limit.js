var TimeLimitedCache = function() {
    this.cache = {}
};

/** 
 * @param {number} key
 * @param {number} value
 * @param {number} duration time until expiration in ms
 * @return {boolean} if un-expired key already existed
 */
TimeLimitedCache.prototype.set = function(key, value, duration) {
    const ret = this.get(key) !== -1 
    this.cache[key] = [value, duration+Date.now()]
    return ret
};

/** 
 * @param {number} key
 * @return {number} value associated with key
 */
TimeLimitedCache.prototype.get = function(key) {
    const val = this.cache[key]
    if (val === undefined || Date.now() > val[1]) return -1
    return val[0]
};

/** 
 * @return {number} count of non-expired keys
 */
TimeLimitedCache.prototype.count = function() {
    let ct = 0
    for (const key of Object.keys(this.cache)) {
        if (this.cache[key] !== undefined && this.cache[key][1] > Date.now()) {
            ct++
        }
    }
    return ct
};

/**
 * const timeLimitedCache = new TimeLimitedCache()
 * timeLimitedCache.set(1, 42, 1000); // false
 * timeLimitedCache.get(1) // 42
 * timeLimitedCache.count() // 1
 */