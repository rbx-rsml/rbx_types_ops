use rbx_types::{Variant, Vector3, Vector3int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector3_with_f32(
    left: &Vector3, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left.x, right),
        operation_fn_f32(left.y, right),
        operation_fn_f32(left.z, right),
    )
}

fn operation_vector3_with_vector3int16(
    left: &Vector3, right: &Vector3int16, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left.x, right.x as f32),
        operation_fn_f32(left.y, right.y as f32),
        operation_fn_f32(left.z, right.z as f32),
    )
}

pub fn operation_vector3_with_vector3(
    left: &Vector3, right: &Vector3, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left.x, right.x),
        operation_fn_f32(left.y, right.y),
        operation_fn_f32(left.z, right.z),
    )
}

impl Operation for Vector3 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::Vector3(with) => Some(Variant::Vector3(operation_vector3_with_vector3(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector3(operation_vector3_with_f32(self, *with, &operation_fn_f32))),
            Variant::Vector3int16(with) => Some(Variant::Vector3(operation_vector3_with_vector3int16(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector3 {}