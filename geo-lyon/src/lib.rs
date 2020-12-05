use bevy_prototype_lyon::prelude::*;

pub trait ToPath {
    fn to_path(&self) -> Path;
}

impl ToPath for geo_types::Polygon<f64> {
    fn to_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();

        polygon_path_builder(self, &mut path_builder);

        path_builder.build()
    }
}

impl ToPath for geo_types::MultiPolygon<f64> {
    fn to_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();

        for polygon in &self.0 {
            polygon_path_builder(polygon, &mut path_builder);
        }

        path_builder.build()
    }
}

fn polygon_path_builder(polygon: &geo_types::Polygon<f64>, path_builder: &mut PathBuilder) {
    ring_path_builder(polygon.exterior(), path_builder);

    for interior in polygon.interiors() {
        ring_path_builder(interior, path_builder);
    }
}

fn ring_path_builder(ring_line_string: &geo_types::LineString<f64>, path_builder: &mut PathBuilder) {
    coords_path_builder(ring_line_string.0.iter().copied(), path_builder);
    path_builder.close();
}

fn coords_path_builder(mut iter: impl Iterator<Item = geo_types::Coordinate<f64>>, path_builder: &mut PathBuilder) {
    let first = match iter.next() {
        Some(coord) => coord,
        None => return,
    };
    path_builder.move_to(point(first.x as f32, first.y as f32));

    for coord in iter {
        path_builder.line_to(point(coord.x as f32, coord.y as f32))
    }
}