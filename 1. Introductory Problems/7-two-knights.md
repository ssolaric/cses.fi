# Two Knights

## Problem

Count the number of ways to place two knights in a k \* k chessboard without attacking each other.

## Solution

A brute force approach gives us TLE for large values of k. Because of this, we try to find a mathematical solution.

In a `k * k` chessboard, where `k > 4`, there is a `(k - 4) * (k - 4)` submatrix at the center. In this submatrix, if we place a knight, then it blocks 8 positions in the board. Therefore, for each position in this submatrix, there are `k^2 - 8 - 1 = k^2 - 9` valid positions for the second knight (we excluded the 8 attacked positions and the position of the first knight). There are `(k - 4)^2` positions in this submatrix.

Using the same approach for the rest of the positions, we find:
- There are `4 * (k - 4)` positions (two rows and two columns adjacent to the submatrix mentioned above) where a knight blocks 6 positions.
- There are `4 * (k - 3)` positions where a knight blocks 4 positions.
- There are `8` positions where a knight blocks 3 positions.
- There are `4` positions where a knight blocks 2 positions.

Summing up all possibilities, we get:
```
(k - 4)^2 * (k^2 - 9) + 4 * (k - 4) * (k^2 - 7) + 4 * (k - 3) * (k^2 - 5) + 8 * (k^2 - 4) + 4 * (k^2 - 3) =
k^4 - 9 * k^2 + 24 * k - 16
```

However, we have double-counted pairs of knights. To fix this, we just divide by two. The final result is:
```
(k^4 - 9 * k^2 + 24 * k - 16) / 2
```
It turns out that this formula also works for `1 <= k <= 4`.