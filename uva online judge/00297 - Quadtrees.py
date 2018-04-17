class QTreeNode:
    def __init__(self):
        self.children = [None, None, None, None]
        self.black = None

    def is_leaf(self):
        return self.black is not None        

    def count_black(self, level=0):
        cnt = 0
        if self.is_leaf() and self.black:
            cnt = (32 // (2 ** level)) ** 2
        elif not self.is_leaf():
            for i in range(4):
                cnt += self.children[i].count_black(level + 1)
        return cnt

    @staticmethod
    def create_from_preorder(string, n=0):
        node = QTreeNode()
        if string[n] == 'p':
            node.children[0], n = QTreeNode.create_from_preorder(string, n + 1)
            node.children[1], n = QTreeNode.create_from_preorder(string, n)
            node.children[2], n = QTreeNode.create_from_preorder(string, n)
            node.children[3], n = QTreeNode.create_from_preorder(string, n)
        elif string[n] == 'e':
            node.black = False
            n += 1
        else:
            node.black = True
            n += 1
        return node, n

    @staticmethod
    def merge(node1, node2):
        if (node1.is_leaf() and  node1.black)  or (node2.is_leaf() and node2.black):
            node = node1 if node1.is_leaf() and  node1.black else node2
        elif node1.is_leaf():
            node = node2
        elif node2.is_leaf():
            node = node1
        else:
            node = QTreeNode()
            for i in range(4):
                node.children[i] = QTreeNode.merge(node1.children[i] , node2.children[i])
        return node

if __name__ == '__main__':
    n = int(input())
    for i in range(n):
        str1 = input().strip()
        str2 = input().strip()
        tree1, _ = QTreeNode.create_from_preorder(str1)
        tree2, _ = QTreeNode.create_from_preorder(str2)
        print("There are",QTreeNode.merge(tree1, tree2).count_black(), "black pixels.")