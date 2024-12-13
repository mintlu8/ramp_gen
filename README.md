# ramp_gen

Generate functions for rust and wgsl similar to Blender's "Color Ramp" node.

## Example

```rust
|x: f32| ramp!(ease [0.0, 5.0], [0.5, 0.0], [7.0, 5.0])
```

## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE(LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT(LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
