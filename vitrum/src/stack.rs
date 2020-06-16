use geometry::{Plane, Vector3D};
use bvh::BoundingVolumeHierarchy;
use num::{Float, Signed};

pub fn stack<T: Plane<S, V>, S, V: Float + Signed>(model: BoundingVolumeHierarchy<T, S, V>) -> BoundingVolumeHierarchy<T, S, V> {
    let min_extents = model.min_extents();
    let max_extents = model.max_extents();
    let size = (max_extents - min_extents).abs().min_value();
    let factor = V::from(2 << 1).unwrap();
    pyramid(min_extents, min_extents + (max_extents - min_extents) * factor, size, &model, min_extents)
}

fn pyramid<T: Plane<S, V>, S, V: Float + Signed>(min: Vector3D<V>, max: Vector3D<V>, size: V,  model: &BoundingVolumeHierarchy<T, S, V>, model_min: Vector3D<V>) -> BoundingVolumeHierarchy<T, S, V> {
    let current_size = (max - min).abs().min_value();

    let two = V::from(2.0).unwrap();
    if current_size <= two * size {
        let shift = min - model_min;
        return model.translate(shift);
    }

    // recursively draw the pyramid

    let center_shift = (max - min) / two;
    //draw the bottom layer

    let center = min + center_shift;

    // bottom left
    let bottom_left = pyramid(min, center, size, model, model_min);

    let bottom_right = pyramid(min + center_shift.proj_x(), center + center_shift.proj_x(), size, model, model_min);

    let top_left = pyramid(min + center_shift.proj_z(), center + center_shift.proj_z(), size, model, model_min);

    let top_right = pyramid(min + center_shift.set_y(V::zero()), center + center_shift.set_y(V::zero()), size, model, model_min);

    // central tower thing
    let top = pyramid(center - (center_shift / two).set_y(V::zero()),
                               center + center_shift - (center_shift / two).set_y(V::zero()),
                               size, model, model_min);

    let bottom_side = <BoundingVolumeHierarchy<T, S, V>>::node(bottom_left, bottom_right);
    let top_side = <BoundingVolumeHierarchy<T, S, V>>::node(top_left, top_right);
    let bottom = <BoundingVolumeHierarchy<T, S, V>>::node(bottom_side, top_side);
    let result = <BoundingVolumeHierarchy<T, S, V>>::node(bottom, top);

    result
}

