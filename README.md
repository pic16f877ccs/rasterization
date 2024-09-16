<div class = "rustdoc-hidden">

 # Rasterization

</div>

This crate provides iterators and adapters for generating sequential coordinates for various shapes of a circle or its parts using Bresenham's algorithm.

## Gallery

Visual examples of different shapes:

|<img title="Filled circle" src="https://raw.githubusercontent.com/pic16f877ccs/image/pixelization/circle.png" alt="" width="" height="">|<img title="Filled long circle" src="https://raw.githubusercontent.com/pic16f877ccs/image/pixelization/circle_long.png" alt="" width="" height="">|<br><img title="Filled top semicircle" src="https://raw.githubusercontent.com/pic16f877ccs/image/pixelization/semicircle_top.png" alt="" width="" height="">|<img title="Filled bottom semicircle" src="https://raw.githubusercontent.com/pic16f877ccs/image/pixelization/semicircle_bottom.png" alt="" width="" height="">|<img title="Filled circle with gradient" src="https://raw.githubusercontent.com/pic16f877ccs/image/pixelization/circle_gradient.png" alt="" width="" height=""></br>|
|:-:|:-:|:-:|:-:|:-:|

## Examples

Here's how to use the crate:

```rust
use rasterization::{Rasterization, SemicircleFilled, DirectionGradient::Bottom};

let radius = 128_usize;
let iter = SemicircleFilled::<i32>::new(radius)
    .circle()
    .offset(radius as i32, radius as i32);

let iter = SemicircleFilled::<i32>::new(radius)
    .circle_long(-128, 0)
    .offset((radius * 2) as i32, radius as i32);

let iter = SemicircleFilled::<i32>::new(radius)
    .semicircle_top()
    .offset(radius as i32, radius as i32);

let iter = SemicircleFilled::<i32>::new(radius)
    .semicircle_bottom()
    .offset(radius as i32, radius as i32);

let iter = SemicircleFilled::<i32>::new(radius)
    .circle()
    .offset(radius as i32, radius as i32)
    .gradient(radius as i32, radius * 2, Bottom(colorous::BROWN_GREEN));
```

<div class = "rustdoc-hidden">

## Installation
Add the following to your Cargo.toml:

```toml
[dependencies]
rasterization = "0.2.0"
```
## License
This project is licensed under the MIT License.

</div>
