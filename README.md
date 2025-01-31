# trashmap

A simple (and bad) hash map implementation for educational purposes.
There are lots of missing features but the goal is to make the API
compatible with `std::collections::HashMap`. 

```rust
use trashmap::separate_chaining::HashMap;

let mut map = HashMap::new();
map.insert("key", 10);
assert_eq!(map.get("key"), Some(&10));
map.insert("key", 20);
assert_eq!(map.get("key"), Some(&20));
map.remove("key");
assert_eq!(map.get("key"), None);
```
