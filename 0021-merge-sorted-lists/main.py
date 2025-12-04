# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        one = list1
        two = list2

        def smaller(l1, l2):
            return True if l2 is None or l2.val is None or (l1 is not None and l1.val < l2.val) else False

        head = None
        tail = None
        while one is not None or two is not None:
            if smaller(one, two):
                newNode = ListNode(one.val)
                one = one.next
            else:
                newNode = ListNode(two.val)
                two = two.next
            
            if head is None: 
                head = newNode
                tail = newNode
            else:
                tail.next = newNode
                tail = tail.next
        
        return head
                

        