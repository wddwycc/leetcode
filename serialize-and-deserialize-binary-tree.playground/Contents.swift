import Foundation

/**
 * Definition for a binary tree node.
 */
public class TreeNode {
    public var val: Int
    public var left: TreeNode?
    public var right: TreeNode?
    public init(_ val: Int) {
        self.val = val
        self.left = nil
        self.right = nil
    }
}

class Codec {
    func serialize(_ root: TreeNode?) -> String {
        if let root = root {
            return """
            {
                "val": \(root.val),
                "left": \(serialize(root.left)),
                "right": \(serialize(root.right))
            }
            """
        } else {
            return "null"
        }
    }

    func deserialize(_ node: [String: Any]) -> TreeNode {
        let res = TreeNode(node["val"] as! Int)
        if let left = node["left"] as? [String: Any] {
            res.left = deserialize(left)
        }
        if let right = node["right"] as? [String: Any] {
            res.right = deserialize(right)
        }
        return res
    }

    func deserialize(_ data: String) -> TreeNode? {
        if data == "null" { return nil }
        let objc = try! JSONSerialization.jsonObject(with: data.data(using: .utf8)!, options: []) as! [String: Any]
        return deserialize(objc)
    }
}

// Your Codec object will be instantiated and called as such:
// var codec = Codec()
// codec.deserialize(codec.serialize(root))
print("hello world")
