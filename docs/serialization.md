# Serialization

I've tryied to use Serde as serialization method but (unfortunatelly) Petgraph do not implement it.

Anyway the command to use are:
```
extern crate serde;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
```