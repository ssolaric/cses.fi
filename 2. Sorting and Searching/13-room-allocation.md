# [Room Allocation](https://cses.fi/problemset/task/1164/)

## Problem

`n` customers come to a hotel, each customer has an arrival and departure day.
Two customers can stay in the same room if the departure day of the first customer is earlier than the arrival day of the second customer.
Find the minimum number of rooms needed.

## Solution

This problem can be solved using the 1D Sweep Line technique, similar to the [Restaurant Customers](https://cses.fi/problemset/task/1619) problem. When using this technique, instead of saving intervals in a vector, we save a discrete timestamp and an _accumulator_ for each _event_.

- A `+1` accumulator represents an _start interval_ event.
- A `-1` accumulator represents an _end interval_ event.

We sort this array of events in ascending order by their timestamps.
**NOTE**: If two events occur at the same time, consider the "start of an interval" event (`+1`) first. We do this in order to meet the `Two customers can stay in the same room if the departure day of the first customer is earlier than the arrival day of the second customer` problem restriction.

However, we are also required to output a valid set of rooms allocated to the `n` customers. These rooms must be printed in the same order as in the input.

For each event, we also save an index representing its interval position in the input order.
After finding the minimum number of rooms (`min_rooms`), we traverse again the events array. For each event, if it has a `+1` accumulator, then a room gets assigned to its corresponding interval. If it has a `-1` accumulator, the room gets unassigned.

A simple and efficient way to assign and unassign rooms is to use a `unassigned_rooms` priority queue initialized with rooms numbered from `1` to `min_rooms`. If the priority queue is implemented as a min-heap, then we will always assign the least numbered room, which allows us to output the room numbers in the same order as the problem sample output.
