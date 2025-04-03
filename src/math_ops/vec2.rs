use rbx_types::{Variant, Vector2, Vector2int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector2_with_f32(
    left: Vector2, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left.x, right),
        operation_fn_f32(left.y, right)
    )
}

fn operation_vector2_with_vector2int16(
    left: Vector2, right: Vector2int16, 
    operation_fn_f32: &OperationFn<f32>,
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left.x, right.x as f32),
        operation_fn_f32(left.y, right.y as f32)
    )
}

fn operation_vector2_with_vector2(
    left: Vector2, right: Vector2, 
    operation_fn_f32: &OperationFn<f32>,
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left.x, right.x),
        operation_fn_f32(left.y, right.y)
    )
}

impl Operation for Vector2 {
    fn operation(
        self, with: Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::Vector2(with) => Some(Variant::Vector2(operation_vector2_with_vector2(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector2(operation_vector2_with_f32(self, with, &operation_fn_f32))),
            Variant::Vector2int16(with) => Some(Variant::Vector2(operation_vector2_with_vector2int16(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector2 {}