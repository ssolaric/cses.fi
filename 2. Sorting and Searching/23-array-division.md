# [Array Division](https://cses.fi/problemset/task/1085)

## Problem

You are given an array containing `n` positive integers.

Your task is to divide the array into `k` subarrays so that the maximum sum in a subarray is as small as possible.

## Solution

This is a **_Binary search the answer_** problem!

Let `x` be a possible maximum sum of a subarray in an array:

- `x` increases when there are _less_ subarrays.
- `x` decreases when there are _more_ subarrays.

This implies that there is some monotonic function which maps `x` to the number of subarrays used.

Then the problem can be transformed into searching for a _reasonable_ (minimum) number of subarrays for a given `x` and then comparing it with `k`.

Let `f(x)` be this function. A `x` is a valid solution if `f(x) <= k`. We binary search for `x` using the maximum element of the array as a lower bound.

`f(x)` can be computed greedily. Iterate over the array while keeping a running sum and a subarray counter. On each iteration, try to add an array element to the running sum. If it is possible (the running sum is less than or equal to `x`), then keep expanding the subarray. Otherwise, create a new subarray. Given that `x` is always greater than or equal to all the elements of the array, it is guaranteed that we can always divide the array into non-empty subarrays.
