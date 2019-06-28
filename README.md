# sRGB `glutin` test

Minimal example to test a potential bug of `glutin`.
See [this issue](https://github.com/rust-windowing/glutin/issues/1175) for more information.

When executing this with `cargo run`, there are a few data points of interest:

- The first two lines of output: the format of the framebuffer. For me it says
  ```
  out: 35904
      srgb
  ```
- The next chunk of output: what `glutin` thinks the framebuffer format is. The
  interesting part is the `srgb` field, which for me is `false`.
- The color of the output. Take a screenshot or somehow inspect the color.
  It *should* be `#808080`. But it's probably `#bbbbbb` or something like that.
