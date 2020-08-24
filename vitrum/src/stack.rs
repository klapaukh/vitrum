use geometry::{Plane, Vec3};
use bvh::BoundingVolumeHierarchy;
use num::{Float, Signed};

pub fn stack<T: Plane>(model: BoundingVolumeHierarchy<T>) -> BoundingVolumeHierarchy<T> {
    let min_extents = model.min_extents();
    let max_extents = model.max_extents();
    let size = (max_extents - min_extents).abs().min();
    let factor = (2 << 1) as f64;
    pyramid(min_extents, min_extents + (max_extents - min_extents) * factor, size, &model, min_extents)
}

fn pyramid<T: Plane>(min: Vec3, max: Vec3, size: f64,  model: &BoundingVolumeHierarchy<T>, model_min: Vec3) -> BoundingVolumeHierarchy<T> {
    let current_size = (max - min).abs().min();

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

    let center_x = Vec3::new(center_shift.x, 0.0, 0.0);
    let center_y = Vec3::new(0.0, center_shift.y, 0.0);
    let center_z = Vec3::new(0.0, 0.0, center_shift.z);
    let center_xz = Vec3::new(center_shift.x, 0.0, center_shift.z);

    let bottom_right = pyramid(min + center_x, center + center_x, size, model, model_min);

    let top_left = pyramid(min + center_z, center + center_z, size, model, model_min);

    let top_right = pyramid(min + center_xz, center + center_xz, size, model, model_min);

    // central tower thing
    let top = pyramid(center - (center_xz / 2.0),
                               center + center_shift - (center_xz / 2.0),
                               size, model, model_min);

    let bottom_side = <BoundingVolumeHierarchy<T>>::node(bottom_left, bottom_right);
    let top_side = <BoundingVolumeHierarchy<T>>::node(top_left, top_right);
    let bottom = <BoundingVolumeHierarchy<T>>::node(bottom_side, top_side);
    <BoundingVolumeHierarchy<T>>::node(bottom, top)
}

