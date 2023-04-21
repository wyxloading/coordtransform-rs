const PI: f64 = std::f64::consts::PI;
const X_PI: f64 = PI * 3000.0 / 180.0;
const OFFSET: f64 = 0.00669342162296594323;
const AXIS: f64 = 6378245.0;

// BD09 -> GCJ02
// 百度坐标系 -> 火星坐标系
pub fn bd09_to_gcj02(lon: f64, lat: f64) -> (f64, f64) {
    let x = lon - 0.0065;
    let y = lat - 0.006;

    let z = (x * x + y * y).sqrt() - 0.00002 * (y * X_PI).sin();
    let theta = y.atan2(x) - 0.000003 * (x * X_PI).cos();

    let g_lon = z * theta.cos();
    let g_lat = z * theta.sin();

    (g_lon, g_lat)
}

// GCJ02 -> BD09
// 火星坐标系 -> 百度坐标系
pub fn gcj02_to_bd09(lon: f64, lat: f64) -> (f64, f64) {
    let z = (lon * lon + lat * lat).sqrt() + 0.00002 * (lat * X_PI).sin();
    let theta = lat.atan2(lon) + 0.000003 * (lon * X_PI).cos();

    let bd_lon = z * theta.cos() + 0.0065;
    let bd_lat = z * theta.sin() + 0.006;

    (bd_lon, bd_lat)
}

// WGS84 -> GCJ02
// WGS84坐标系 -> 火星坐标系
pub fn wgs84_to_gcj02(lon: f64, lat: f64) -> (f64, f64) {
    if is_out_of_china(lon, lat) {
        return (lon, lat);
    }

    let (mg_lon, mg_lat) = delta(lon, lat);

    (mg_lon, mg_lat)
}

// GCJ02 -> WGS84
// 火星坐标系 -> WGS84坐标系
pub fn gcj02_to_wgs84(lon: f64, lat: f64) -> (f64, f64) {
    if is_out_of_china(lon, lat) {
        return (lon, lat);
    }

    let (mg_lon, mg_lat) = delta(lon, lat);

    (lon * 2.0 - mg_lon, lat * 2.0 - mg_lat)
}

// BD09 -> WGS84
// 百度坐标系 -> WGS84坐标系
pub fn bd09_to_wgs84(lon: f64, lat: f64) -> (f64, f64) {
    let (lon, lat) = bd09_to_gcj02(lon, lat);
    gcj02_to_wgs84(lon, lat)
}

// WGS84 -> BD09
// WGS84坐标系 -> 百度坐标系
pub fn wgs84_to_bd09(lon: f64, lat: f64) -> (f64, f64) {
    let (lon, lat) = wgs84_to_gcj02(lon, lat);
    gcj02_to_bd09(lon, lat)
}

fn delta(lon: f64, lat: f64) -> (f64, f64) {
    let (mut d_lat, mut d_lon) = transform(lon - 105.0, lat - 35.0);
    let rad_lat = lat / 180.0 * PI;
    let mut magic = rad_lat.sin();
    magic = 1.0 - OFFSET * magic * magic;
    let sqrt_magic = magic.sqrt();

    d_lat = (d_lat * 180.0) / ((AXIS * (1.0 - OFFSET)) / (magic * sqrt_magic) * PI);
    d_lon = (d_lon * 180.0) / (AXIS / sqrt_magic * rad_lat.cos() * PI);

    let mg_lat = lat + d_lat;
    let mg_lon = lon + d_lon;

    (mg_lon, mg_lat)
}

fn transform(lon: f64, lat: f64) -> (f64, f64) {
    let lon_lat = lon * lat;
    let abs_x = lon.abs().sqrt();
    let lon_pi = lon * PI;
    let lat_pi = lat * PI;
    let d = 20.0 * (6.0 * lon_pi).sin() + 20.0 * (2.0 * lon_pi).sin();
    let mut x = d;
    let mut y = d;
    x += 20.0 * lat_pi.sin() + 40.0 * (lat_pi / 3.0).sin();
    y += 20.0 * lon_pi.sin() + 40.0 * (lon_pi / 3.0).sin();
    x += 160.0 * (lat_pi / 12.0).sin() + 320.0 * (lat_pi / 30.0).sin();
    y += 150.0 * (lon_pi / 12.0).sin() + 300.0 * (lon_pi / 30.0).sin();
    x *= 2.0 / 3.0;
    y *= 2.0 / 3.0;
    x += -100.0 + 2.0 * lon + 3.0 * lat + 0.2 * lat * lat + 0.1 * lon_lat + 0.2 * abs_x;
    y += 300.0 + lon + 2.0 * lat + 0.1 * lon * lon + 0.1 * lon_lat + 0.1 * abs_x;
    (x, y)
}

fn is_out_of_china(lon: f64, lat: f64) -> bool {
    lon < 72.004 || lon > 137.8347 || lat < 0.8293 || lat > 55.8271
}
