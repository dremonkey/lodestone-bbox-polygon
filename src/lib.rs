/// The main crate for lodestone-bbox-polygon
/// 
/// ## Overview
/// 
/// Takes a bounding box and creates a FeaturePolygon GeoJson feature.
/// Inspired by [turf-bbox-polygon](https://github.com/Turfjs/turf-bbox-polygon).

// Third party crates
extern crate lodestone_polygon;

use lodestone_polygon::FeaturePolygon;

/// ## Arguments
/// 
/// * `bbox` - a Vec<T> of bounding box coordinates in the form `[xLow, yLow, xHigh, yHigh]`
pub extern fn bbox_polygon(bbox: Vec<f64>) -> FeaturePolygon {

  let low_left = vec![bbox[0], bbox[1]];
  let top_left = vec![bbox[0], bbox[3]];
  let top_right = vec![bbox[2], bbox[3]];
  let low_right = vec![bbox[2], bbox[1]];
  let coords = vec![vec![low_left.clone(), low_right, top_right, top_left, low_left]];
  
  FeaturePolygon::new(coords)
}

#[cfg(test)]
mod tests {
  use super::bbox_polygon;

  #[test]
  fn test_valid_geojson() {
    let poly = bbox_polygon(vec![0.0, 0.0, 10.0, 10.0]);
    let coords = poly.coordinates();

    assert_eq!(coords[0].len(), 5);
    assert_eq!(coords[0].first(), coords[0].last());
  }
}
