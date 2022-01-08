from typing import List
from unittest import TestCase

class Solution:
    def carPooling(self, trips: List[List[int]], capacity: int) -> bool:
        days = []
        for num, fr, to in trips:
            days.append((fr, num))
            days.append((to, -num))
        current_num = 0
        current_day = None
        for day, num in sorted(days):
            if current_day != day:
                if current_num > capacity:
                    return False
                current_day = day
            current_num += num
        return current_num <= capacity
        
class CarPoolingTest(TestCase):
    def test_1(self) -> None:
        self.assertFalse(Solution().carPooling([[2,1,5],[3,3,7]], 4)) 

    def test_2(self) -> None:
        self.assertTrue(Solution().carPooling([[2,1,5],[3,3,7]], 5)) 
