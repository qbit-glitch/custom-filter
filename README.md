# custom-filter

A simple Rust program that filters a collection of integers based on a specified condition.

## Overview

The `custom-filter` program demonstrates how to filter elements from a vector of integers. It allows you to specify a filtering condition using a `FilterCondition` struct, making it easy to determine which elements should be included in the filtered results.

## Features

- Define a filtering condition with a custom threshold.
- Filter a collection of integers based on the defined condition.
- Easy to extend for additional filtering criteria.

## Code Structure

### Main Function

The `main` function initializes a `FilterCondition` and a collection of integers, then prints the filtered results.

```rust
fn main() {
    let filter_condition = FilterCondition { num: 87 };
    let collection = vec![56, 78, 98, 87, 24, 65, 87];

    println!("{:?}", custom_filter(collection, filter_condition));
}
```

### FilterCondition Struct

The `FilterCondition` struct encapsulates the filtering criteria:

```rust
struct FilterCondition {
    num: i32,
}
```

#### is_match Method

This method checks if a given number meets the filter condition:

```rust
impl FilterCondition {
    fn is_match(&self, n: &i32) -> bool {
        *n >= self.num
    }
}
```

### custom_filter Function

The `custom_filter` function takes a vector of integers and a `FilterCondition` and returns a new vector containing only the elements that match the condition:

```rust
fn custom_filter(collection: Vec<i32>, filter: FilterCondition) -> Vec<i32> {
    let mut new_collection: Vec<i32> = Vec::new();
    for number in collection {
        if filter.is_match(&number) {
            new_collection.push(number)
        }
    }
    new_collection
}
```

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/qbit-glitch/custom-filter.git
   ```

2. Navigate to the project directory:

   ```bash
   cd custom-filter
   ```

3. Build and run the program:

   ```bash
   cargo run
   ```

## Example

Given the collection `[56, 78, 98, 87, 24, 65, 87]` and a filter condition of `num = 87`, the output will be:

```
[98, 87, 87]
```

## Contributing

Contributions are welcome! If you'd like to enhance the functionality or fix issues, please feel free to submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
