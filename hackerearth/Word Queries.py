class TrieNode:
    def __init__(self):
        self.children = [None] * 26
        self.cnt = 0

def insert(trie, string):
    for value in (ord(ch) - 97 for ch in string):
        if trie.children[value] is None:
            trie.children[value] = TrieNode()
        trie = trie.children[value]
        trie.cnt += 1 

def query(trie, string):
    for value in (ord(ch) - 97 for ch in string):
        if trie.children[value] is None:
            return 0
        else:
            trie = trie.children[value]
    return trie.cnt

if __name__ == '__main__':
    n, q = map(int, input().split())
    trie = TrieNode()
    for i in range(n):
       insert(trie, input())
    for i in range(q):
        print(query(trie, input()))
        

