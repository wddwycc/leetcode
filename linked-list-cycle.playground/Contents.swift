import Foundation


 // Definition for singly-linked list.
 public class ListNode {
     public var val: Int
     public var next: ListNode?
     public init(_ val: Int) {
         self.val = val
         self.next = nil
     }
 }


class Solution {
    func hasCycle(_ head: ListNode?) -> Bool {
        var set: Set<UnsafeMutableRawPointer> = Set()
        var cur: Optional<ListNode> = head
        while let v = cur {
            let addr = Unmanaged.passUnretained(v).toOpaque()
            let (inserted, _) = set.insert(addr)
            if !inserted {
                return true
            }
            cur = v.next
        }
        return false
    }
}
