coordtransform
----

提供百度坐标系(bd-09)、火星坐标系(国测局坐标系、gcj02)、WGS84坐标系的相互转换

## Usage

Add the following to your `cargo.toml`:

```toml
[dependencies]
coordtransform-rs = "0.1"
```

```rust
use coordtransform_rs::gcj02_to_wgs84;

let (lon, lat) = gcj02_to_wgs84(113, 25);
println!("{}, {}", lon, lat);
```