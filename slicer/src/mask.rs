use geo::Area;

use crate::{plotter::polygon_operations::PolygonOperations, Object};

pub fn crop_masks(objects: &[Object], masks: &mut Vec<Object>, max_height: f32) {
    for mask_object in masks.iter_mut() {
        mask_object
            .layers
            .iter_mut()
            .enumerate()
            .for_each(|(index, layer)| {
                let mut remaining_polygon = layer.main_polygon.clone();
                for object in objects.iter() {
                    if let Some(layer) = object.layers.get(index) {
                        remaining_polygon = remaining_polygon.difference_with(&layer.main_polygon);
                    }
                }

                layer.main_polygon = layer.main_polygon.difference_with(&remaining_polygon);
                layer.remaining_area = layer.main_polygon.clone();
            });

        mask_object.layers.retain(|layer| {
            layer.main_polygon.unsigned_area() > f32::EPSILON || layer.top_height <= max_height
        });
    }
}

pub fn randomize_mask_underlaps(masks: &mut Vec<Object>) {
    for mask_object in masks.iter_mut() {
        mask_object.layers.iter_mut().for_each(|layer| {
            let inset: f32 = rand::random::<f32>() * 2.0;

            layer.main_polygon = layer.main_polygon.offset_from(-inset);
            layer.remaining_area = layer.main_polygon.clone();
        });
    }
}
