from __future__ import annotations
from typing import Optional, Any
from collections import deque

class Node:
    def __init__(self, val: int = 0, left: Optional[Node] = None, right: Optional[Node] = None, next: Optional[Node] = None) -> None:
        self.val = val
        self.left = left
        self.right = right
        self.next = next
    
    def __eq__(self, o: Any) -> bool:
        return isinstance(o, Node) and self.val == o.val and self.left == o.left and self.right == o.right and self.next == o.next

class Solution:
    def connect(self, root: Optional[Node]) -> Optional[Node]:
        if root is not None:
            queue = deque([(root, 0)])
            last_level, last_next = 0, None
            while queue:
                node, level = queue.popleft()
                if level > last_level:
                    last_level = level
                    last_next = None
                node.next = last_next
                last_next = node
                if node.right is not None:
                    queue.append((node.right, level + 1))
                if node.left is not None:
                    queue.append((node.left, level + 1))
        return root
