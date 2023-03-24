class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children if children is not None else []

class Solution:
    def diameter(self, root):
        def rec(node):
            if node is None:
                return (0, 0)
            max_diameter = 0
            max_depths = [0, 0]
            for child in node.children:
                (child_max_diameter, child_max_depth) = rec(child)
                if child_max_diameter > max_diameter:
                    max_diameter = child_max_diameter
                if child_max_depth > max_depths[0]:
                    max_depths[1] = max_depths[0]
                    max_depths[0] = child_max_depth
                elif child_max_depth > max_depths[1]:
                    max_depths[1] = child_max_depth
            if max_depths[0] + max_depths[1] > max_diameter:
                max_diameter = max_depths[0] + max_depths[1]
            return (max_diameter, max_depths[0] + 1)

        return rec(root)[0]
