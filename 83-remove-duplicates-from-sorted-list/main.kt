/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */
class Solution {
    fun deleteDuplicates(head: ListNode?): ListNode? {
        val head = head
        var tail = head
        while (tail?.next != null) {
            if (tail?.next?.`val` == tail?.`val`) {
                tail?.next = tail?.next?.next
            } else {
                tail = tail?.next
            }
        }
        return head
    }
}