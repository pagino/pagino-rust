## Pagino

Handle pagination's logic

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
pagino = "1.0.2"
```

## Example

```rust
use pagino::Pagino;

fn main(){
  let mut pagino = Pagino::new(true, true, true, true, 1, 10, 1, 1);

  let pages: Vec<i32> = pagino.get_pages();
  /*
    pages: [-1, -2, 1, 2, 3, 4, 5, -4, 10, -5, -6];
  */

  pagino.set_page(10);
  let pages: Vec<i32> = pagino.get_pages();
  /*
    pages: [-1, -2, 1, -3, 6, 7, 8, 9, 10, -5, -6];
  */

  pagino.set_page(5);
  let pages: Vec<i32> = pagino.get_pages();
  /*
    pages: [-1, -2, 1, -3, 4, 5, 6, -4, 10, -5, -6];
  */
}

```

## What are negative numbers?
All negative numbers are navigate element

```rust
-1  First
-2  Previous
-3  Start ellipsis
-4  End ellipsis
-5  Next
-6  Last
```

## Pagino config
```rust
show_first: bool
show_previous: bool
show_next: bool
show_last: bool
page: i32
count: i32
sibling_count: i32
boundary_count: i32
```