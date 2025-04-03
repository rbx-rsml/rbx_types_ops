use rbx_types::{Variant, Vector3, Vector3int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector3int16_with_f32(
    left: Vector3int16, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left.x as f32, right) as i16,
        operation_fn_f32(left.y as f32, right) as i16,
        operation_fn_f32(left.z as f32, right) as i16,
    )
}

fn operation_vector3int16_with_vector3(
    left: Vector3int16, right: Vector3, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left.x as f32, right.x) as i16,
        operation_fn_f32(left.y as f32, right.y) as i16,
        operation_fn_f32(left.z as f32, right.z) as i16,
    )
}

fn operation_vector3int16_with_vector3int16(
    left: Vector3int16, right: Vector3int16, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left.x as f32, right.x as f32) as i16,
        operation_fn_f32(left.y as f32, right.y as f32) as i16,
        operation_fn_f32(left.z as f32, right.z as f32) as i16,
    )
}

impl Operation for Vector3int16 {
    fn operation(
        self, with: Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::Vector3int16(with) => Some(Variant::Vector3int16(operation_vector3int16_with_vector3int16(self, with, &operation_fn_f32))),
            Variant::Vector3(with) => Some(Variant::Vector3int16(operation_vector3int16_with_vector3(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector3int16(operation_vector3int16_with_f32(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector3int16 {}