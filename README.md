# sRGB `glutin` test

Minimal example to test a potential bug of `glutin`.
See [this issue](https://github.com/rust-windowing/glutin/issues/1175) for more information.

When executing this with `cargo run`, there are two data points of interest:

- The output on stdout
- The color of the output. Take a screenshot or somehow inspect the color.
  It *should* be `#808080`. But it's probably `#bbbbbb` or something like that.


## Results

<table>
  <thead>
    <tr>
      <th></th>
      <th><code>with_srgb(false)</code></th>
      <th><code>with_srgb(true)</code></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Ubuntu 18.04</td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>Windows 10</td>
      <td><code>#bdbdbd</code><br><pre>What OpenGL thinks: linear (9729)
What glutin thinks: srgb = true</pre></td>
      <td><code>#bdbdbd</code><br><pre>What OpenGL thinks: linear (9729)
What glutin thinks: srgb = true</pre></td>
    </tr>
  </tbody>
</table>


### Platform information
#### Ubuntu 18.04
