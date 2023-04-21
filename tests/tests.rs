use coordtransform_rs::*;

#[test]
fn test_bd09_to_gcj02() {
    let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
    assert_eq!((lon, lat), (116.39762729119315, 39.90865673957631))
}

#[test]
fn test_gcj02_to_bd09() {
    let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
    assert_eq!((lon, lat), (116.41036949371029, 39.92133699351021))
}

#[test]
fn test_wgs84_to_gcj02() {
    let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
    assert_eq!((lon, lat), (116.41024449916938, 39.91640428150164))
}

#[test]
fn test_gcj02_wgs84() {
    let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
    assert_eq!((lon, lat), (116.39775550083061, 39.91359571849836))
}

#[test]
fn test_bd09_to_wgs84() {
    let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
    assert_eq!((lon, lat), (116.3913836995125, 39.907253214522164))
}

#[test]
fn test_wgs84_to_bd09() {
    let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
    assert_eq!((lon, lat), (116.41662724378733, 39.922699552216216))
}