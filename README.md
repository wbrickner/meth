# [`Meth`](https://crates.io/crates/meth): stay awake!

![Graphic shows two cursors moving with pixel-art motion blur](https://github.com/wbrickner/meth/raw/main/images/optimized_header.jpg)

Keeps computers awake!

I often run long simulations, and like to walk away from my computer.

- I don't want to change my power settings, 
- I only want *one program* to keep the computer from sleeping when it is running, or just when it is performing a specific task.

# Usage

A [`Meth`](https://docs.rs/meth) instance will keep the computer awake until dropped.

```rust
// computer will not sleep until
// `important_computation()` completes.
{
  let meth = Meth::new();
  important_computation();
}

// `meth` dropped,
// computer might enter sleep now.
```

# OS Support
## Windows + macOS
Should just work

## Linux
Ensure `libxdo-dev` is installed:

- **Ubuntu**: `sudo apt-get install libxdo-dev`

- **Arch**: `pacman -S xdotool`

- **AWS Linux, CentOS, etc.**: `yum install libxdo`

# How it works

![GIF showing a macOS cursor moving in a square](https://github.com/wbrickner/meth/raw/main/images/optimized_demo.gif)

It just... slightly wiggles the mouse occasionally (using [`mouse-rs`](https://crates.io/crates/mouse-rs)).

If the mouse is left unmoved for 30 seconds, `meth` will move it, tracing out a square.

# [MIT License](https://opensource.org/licenses/MIT)
```text
Permission is hereby granted, free of charge, to any person obtaining a 
copy of this software and associated documentation files (the "Software"), 
to deal in the Software without restriction, including without limitation 
the rights to use, copy, modify, merge, publish, distribute, sublicense, 
and/or sell copies of the Software, and to permit persons to whom the 
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in 
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS 
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE 
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER 
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING 
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
DEALINGS IN THE SOFTWARE.
```