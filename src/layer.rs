use geo::contains::Contains;
use geo::bounding_rect::BoundingRect;
use pathfinder_canvas::ColorU;

#[derive(Clone, Debug)]
pub struct Layers {
    pub data: Vec<Layer>,
    pub bounding_rect: Option<geo::Rect<f64>>,
}

impl Layers {
    pub fn new() -> Layers {
        Layers {
            data: vec![],
            bounding_rect: None,
        }
    }

    pub fn containing_coord(&self, coord: geo::Coordinate<f64>) -> Vec<Layer> {
        let bounding_rect = match self.bounding_rect {
            Some(b) => b,
            None => return vec![],
        };

        if !bounding_rect.contains(&coord) {
            return vec![];
        }

        self.data.iter().filter(|layer| layer.contains_coord(coord)).cloned().collect()
    }

    pub fn add(&mut self, geometry: geo::Geometry<f64>, metadata: Option<Metadata>) {
        let layer = Layer::from_geometry(geometry, metadata);
        self.bounding_rect = Some(if let Some(r) = self.bounding_rect {
            bbox_merge(r, layer.bounding_rect)
        } else {
            layer.bounding_rect
        });
        self.data.push(layer);
    }
}

fn geometry_bounding_rect(geometry: &geo::Geometry<f64>) -> geo::Rect<f64> {
    match geometry {
        geo::Geometry::LineString(line_string) => line_string.bounding_rect().unwrap(),
        geo::Geometry::Polygon(polygon) => polygon.bounding_rect().unwrap(),
        geo::Geometry::MultiLineString(multi_line_string) => {
            multi_line_string.bounding_rect().unwrap()
        }
        geo::Geometry::MultiPolygon(multi_polygon) => multi_polygon.bounding_rect().unwrap(),
        _ => unimplemented!(),
    }
}

fn bbox_merge(a: geo::Rect<f64>, b: geo::Rect<f64>) -> geo::Rect<f64> {
    geo::Rect::new(
        geo::Coordinate {
            x: a.min().x.min(b.min().x),
            y: a.min().y.min(b.min().y),
        },
        geo::Coordinate {
            x: a.max().x.max(b.max().x),
            y: a.max().y.max(b.max().y),
        },
    )
}

pub type Metadata = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct Layer {
    pub geometry: geo::Geometry<f64>,
    pub bounding_rect: geo::Rect<f64>,
    pub color: ColorU,
    pub metadata: Metadata,
}

impl Layer {
    pub fn contains_coord(&self, coord: geo::Coordinate<f64>) -> bool {
        self.bounding_rect.contains(&geo::Point(coord))
            && self.geometry.contains(&geo::Point(coord))
    }

    pub fn from_geometry(geometry: geo::Geometry<f64>, metadata: Option<Metadata>) -> Self {
        Layer {
            bounding_rect: geometry_bounding_rect(&geometry),
            geometry: geometry,
            color: crate::color::next(),
            metadata: metadata.unwrap_or_else(|| serde_json::Map::new()),
        }
    }
}
