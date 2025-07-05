# Shapley


[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/shapley)](https://crates.io/crates/shapley)
[![PyPI - Version](https://img.shields.io/pypi/v/shapleyrs)](https://pypi.org/project/shapleyrs/)
[![docs.rs](https://img.shields.io/docsrs/shapley)](https://docs.rs/shapley/latest/shapley/)


[`shapley`](https://github.com/shenxiangzhuang/shapley)
is a fast Shapley value calculator written in rust.

## Install

- Python: `pip install shapleyrs`
- Rust: `cargo add shapley`


## Quick Start

### Python

```python
from shapleyrs import Shapley

shapley = Shapley(players=[1, 2], coalition_worth={
    (): 0,
    (1,): 10,
    (2,): 20,
    (1, 2): 30,
})

print(shapley.shapley_value(1)) # 10
print(shapley.shapley_value(2)) # 20
```


### Rust

```rust
use shapley::Shapley;

let shapley = Shapley::new(players: [1, 2], coalition_worth: {
    (): 0,
    (1,): 10,
    (2,): 20,
    (1, 2): 30,
});

println!("{}", shapley.shapley_value(1)); // 10
println!("{}", shapley.shapley_value(2)); // 20
```
