/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode a = l1;
        ListNode b = l2;
        ListNode tail = null;
        ListNode head = null;
        int carry = 0;
        while (a != null || b != null || carry != 0) {
            int aVal = a == null ? 0 : a.val;
            int bVal = b == null ? 0 : b.val;
            int sum = aVal+bVal+carry;
            carry= sum / 10;
            if (tail != null){
                tail.next = new ListNode(sum%10);
                tail = tail.next;
            }else {
                head = new ListNode(sum%10);
                tail = head;
            }
            a = a==null?a:a.next;
            b = b==null?b:b.next;
        }
        return head;
        
    }
}