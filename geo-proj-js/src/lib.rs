#![warn(clippy::unwrap_used, clippy::unimplemented, clippy::expect_used)]

use geo::algorithm::map_coords::MapCoordsInplace;
use std::{error, fmt};
use wasm_bindgen::JsCast;

#[derive(Debug)]
struct CouldNotProjectError;

impl fmt::Display for CouldNotProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Failed to transform with proj.js")
    }
}

impl error::Error for CouldNotProjectError {}

pub fn transform(
    geometry: &mut geo::Geometry<f64>,
    source_crs: &str,
    target_crs: &str,
) -> Result<(), Box<dyn error::Error>> {
    let proj4 = web_sys::window()
        .ok_or(CouldNotProjectError)?
        .get("proj4")
        .ok_or(CouldNotProjectError)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| CouldNotProjectError)?;
    let projector = proj4
        .call2(
            &wasm_bindgen::JsValue::UNDEFINED,
            &source_crs.into(),
            &target_crs.into(),
        )
        .map_err(|_| CouldNotProjectError)?;
    let array = js_sys::Array::new_with_length(2);
    let forward = js_sys::Reflect::get(&projector, &"forward".into())
        .map_err(|_| CouldNotProjectError)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| CouldNotProjectError)?;
    geometry.map_coords_inplace(|(x, y)| {
        array.set(0, wasm_bindgen::JsValue::from_f64(*x));
        array.set(1, wasm_bindgen::JsValue::from_f64(*y));
        let result = forward
            .call1(&wasm_bindgen::JsValue::UNDEFINED, &array)
            .unwrap()
            .dyn_into::<js_sys::Array>()
            .unwrap();
        (
            result.get(0).as_f64().unwrap(),
            result.get(1).as_f64().unwrap(),
        )
    });
    Ok(())
}
