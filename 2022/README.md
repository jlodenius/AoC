## Some bookmarks for later

### Day 12
> Pathfinding using dijkstra algo, used wikipedias definition of
> the algorithm to make my own implementation. It feels slow, come
> back to this in the future and improve performance.

### Day 13
> Using serde json to parse input into a custom Enum `Node`.
> Implementing the `Ord` and `PartialOrd` traits to sort a vector of `Node`
> Implementing a custom `Debug` fn to print `Node`

### Day 15
> Parsing strings with nom. Struggled a bit with performance in part 2.
> Another one to possibly come back to in the future. Not so much Rust
> issues though.

### Day 16
> Stored two-letter strings as byte slices in a tuple struct `struct ValveName([u8; 2])`
> to make use of the Copy trait. Brute forced all possible paths to success with recursion.
> In part two they introduced an extra "player" which made the pool of possible paths
> exponentially larger. Brute force still worked though without any further optimisation.
> Speed dropped from a few ms in part one to about 3 minutes in part two (with a release build...)
> Should definitely come back to this one for improvements. Memoization for one.

### Day 17
> Not sure why part 2 is not giving me the right answer, works for sample input but not the real.
> Leaving this one for now and moving on.
