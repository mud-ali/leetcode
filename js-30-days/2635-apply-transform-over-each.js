/**
 * @param {number[]} arr
 * @param {Function} fn
 * @return {number[]}
 */
var map = function(arr, fn) {
  const newArray = [...arr]
  for (let i=0;i<arr.length;i++) {
    newArray[i] = fn(newArray[i], i);
  }  
  return newArray;
};

