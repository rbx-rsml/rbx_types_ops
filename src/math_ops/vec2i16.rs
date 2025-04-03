use rbx_types::{Variant, Vector2, Vector2int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector2int16_with_f32(
    left: &Vector2int16, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector2int16 {
    Vector2int16::new(
        operation_fn_f32(left.x as f32, right) as i16,
        operation_fn_f32(left.y as f32, right) as i16
    )
}

fn operation_vector2int16_with_vector2(
    left: &Vector2int16, right: &Vector2, 
    operation_fn_f32: &OperationFn<f32>,
) -> Vector2int16 {
    Vector2int16::new(
        operation_fn_f32(left.x as f32, right.x) as i16,
        operation_fn_f32(left.y as f32, right.y) as i16
    )
}

fn operation_vector2int16_with_vector2int16(
    left: &Vector2int16, right: &Vector2int16, 
    operation_fn_f32: &OperationFn<f32>,
) -> Vector2int16 {
    Vector2int16::new(
        operation_fn_f32(left.x as f32, right.x as f32) as i16,
        operation_fn_f32(left.y as f32, right.y as f32) as i16
    )
}

impl Operation for Vector2int16 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::Vector2int16(with) => Some(Variant::Vector2int16(operation_vector2int16_with_vector2int16(self, with, &operation_fn_f32))),
            Variant::Vector2(with) => Some(Variant::Vector2int16(operation_vector2int16_with_vector2(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector2int16(operation_vector2int16_with_f32(self, *with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector2int16 {}