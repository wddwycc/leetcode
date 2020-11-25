/**
 * Definition for a Node.
 * public class Node {
 *     public var val: Int
 *     public var children: [Node]
 *     public init(_ val: Int) {
 *         self.val = val
 *         self.children = []
 *     }
 * }
 */

public class Node {
    public var val: Int
    public var children: [Node]
    public init(_ val: Int) {
        self.val = val
        self.children = []
    }
}


class Solution {
    func preorder(_ root: Node?) -> [Int] {
        guard let root = root else { return [] }
        var res: [Int] = []
        var stack: [(Node, visited: Bool)] = []
        stack.append((root, false))
        while let (node, visited) = stack.popLast() {
            res.append(node.val)
            if !visited {
                var children = node.children
                while let node = children.popLast() {
                    stack.append((node, false))
                }
            }
        }
        return res
    }
}
