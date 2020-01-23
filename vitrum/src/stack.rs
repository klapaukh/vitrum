use geometry::{Plane, Vector3D};
use bvh::BoundingVolumeHierarchy;

pub fn stack<T: Plane<S>, S>(model: BoundingVolumeHierarchy<T, S>) -> BoundingVolumeHierarchy<T, S> {
    let min_extents = model.min_extents();
    let max_extents = model.max_extents();
    let size = (max_extents - min_extents).abs().min_value();
    let factor = (2 << 4) as f32;
    pyramid(min_extents, min_extents + (max_extents - min_extents) * factor, size, &model, min_extents)
}

fn pyramid<T: Plane<S>, S>(min: Vector3D, max: Vector3D, size: f32,  model: &BoundingVolumeHierarchy<T, S>, model_min: Vector3D) -> BoundingVolumeHierarchy<T, S> {
    let current_size = (max - min).abs().min_value();

    if current_size <= 2.0 * size {
        let shift = min - model_min;
        return model.translate(shift);
    }

    // recursively draw the pyramid

    let center_shift = (max - min) / 2.0;
    //draw the bottom layer

    let center = min + center_shift;

    // bottom left
    let bottom_left = pyramid(min, center, size, model, model_min);

    let bottom_right = pyramid(min + center_shift.proj_x(), center + center_shift.proj_x(), size, model, model_min);

    let top_left = pyramid(min + center_shift.proj_z(), center + center_shift.proj_z(), size, model, model_min);

    let top_right = pyramid(min + center_shift.set_y(0.0), center + center_shift.set_y(0.0), size, model, model_min);

    // central tower thing
    let top = pyramid(center - (center_shift / 2.0).set_y(0.0),
                               center + center_shift - (center_shift / 2.0).set_y(0.0),
                               size, model, model_min);

    let bottom_side = <BoundingVolumeHierarchy<T, S>>::node(bottom_left, bottom_right);
    let top_side = <BoundingVolumeHierarchy<T, S>>::node(top_left, top_right);
    let bottom = <BoundingVolumeHierarchy<T, S>>::node(bottom_side, top_side);
    let result = <BoundingVolumeHierarchy<T, S>>::node(bottom, top);

    result
}

