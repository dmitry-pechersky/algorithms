from typing import Optional,List
from unittest import TestCase

class PolyNode:
    def __init__(self, x=0, y=0, next=None):
        self.coefficient = x
        self.power = y
        self.next = next

    def __eq__(self, o):
        return isinstance(o, PolyNode) and self.coefficient == o.coefficient and self.power == o.power and self.next == o.next

    def __repr__(self):
        return f"({self.coefficient} {self.power} {repr(self.next)})"        

class Solution:
    def addPoly(self, poly1: Optional[PolyNode], poly2: Optional[PolyNode]) -> Optional[PolyNode]:
        if poly1 is None:
            return poly2
        elif poly2 is None:
            return poly1
        elif poly1.power > poly2.power:
            return PolyNode(
                poly1.coefficient,
                poly1.power,
                self.addPoly(poly1.next, poly2)
            )
        elif poly1.power < poly2.power:
            return PolyNode(
                poly2.coefficient,
                poly2.power,
                self.addPoly(poly1, poly2.next)
            )
        elif poly1.coefficient == - poly2.coefficient:
            return self.addPoly(poly1.next, poly2.next)
        else:
            return PolyNode(
                poly1.coefficient + poly2.coefficient,
                poly1.power,
                self.addPoly(poly1.next, poly2.next)
            )
            
def list_to_poly(l: List[List[int]]) -> Optional[PolyNode]:
    poly = None
    for i in reversed(range(len(l))):
        poly = PolyNode(l[i][0], l[i][1], poly)
    return poly


class AddPolyTest(TestCase):
    def test_1(self):
        self.assertEqual(
            Solution().addPoly(list_to_poly([[1,1]]), list_to_poly([[1,0]])),
            list_to_poly([[1,1],[1,0]])
        )
        
    def test_2(self):
        self.assertEqual(
            Solution().addPoly(list_to_poly([[2,2],[4,1],[3,0]]), list_to_poly([[3,2],[-4,1],[-1,0]])),
            list_to_poly([[5,2],[2,0]])
        )

    def test_3(self):
        self.assertEqual(
            Solution().addPoly(list_to_poly([[1,2]]), list_to_poly([[-1,2]])),
            list_to_poly([])
        )
