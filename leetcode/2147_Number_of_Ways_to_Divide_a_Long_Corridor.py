from typing import Optional
from unittest import TestCase

class Solution:
    def numberOfWays(self, corridor: str) -> int:
        first_seat, last_seat, seat_cnt = None, None, 0 
        for (i, val) in enumerate(corridor):
            if val == 'S':
                last_seat = i
                seat_cnt += 1
                if first_seat is None:
                    first_seat = i
        if seat_cnt > 0 and seat_cnt % 2 == 0:
            cnt, seat_cnt, cont_plant_cnt = 1, 0, 0
            for i in range(first_seat, last_seat):
                if corridor[i] == 'S':
                    seat_cnt += 1
                    if cont_plant_cnt > 0:
                        cnt = cnt * (cont_plant_cnt + 1) % 1000000007
                        cont_plant_cnt = 0
                elif seat_cnt % 2 == 0:
                    cont_plant_cnt += 1
            return cnt
        return 0
        
class NumberOfWaysTest(TestCase):
    def test_1(self):
        self.assertEqual(Solution().numberOfWays("SSPPSPS"), 3)
        
    def test_2(self):
        self.assertEqual(Solution().numberOfWays("PPSPSP"), 1)

    def test_3(self):
        self.assertEqual(Solution().numberOfWays("S"), 0)

    def test_4(self):
        self.assertEqual(Solution().numberOfWays("SPSPPSSPSSSS"), 6)
