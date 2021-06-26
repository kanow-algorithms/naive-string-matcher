# naive-string-matcher

## Requirements

Before testing this code, please make sure you have:

- **[rust](https://www.rust-lang.org/)**
- **[cargo](https://crates.io/)**

## Installation

Clone repo:
```bash
git clone https://github.com/kanow-algorithms/naive-string-matcher.git
```

## Testing

To run tests run this command in the root of this project:
```bash
cargo test
```

## Description

naive_string_matcher takes two parameters:

- _**text**_ → is a reference to vector of all characters in which you want to find the word
- _**pattern**_ → is a reference to vector of all search word characters

### Example

**Input:**

```rust
let text = vec!['H', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
let pattern = vec!['w', 'o', 'r', 'l', 'd'];
naive_string_matcher(&text, &pattern);
```

**Output:**

```bash
[6]
```

## How it works

On this graphic you can see how naive-string-matcher algorithm works:

<p align="center">
  <img src="https://i.stack.imgur.com/KCy6H.jpg" alt="NaiveStringMatcher"/>
</p>
