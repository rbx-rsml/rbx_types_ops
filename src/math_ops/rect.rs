use rbx_types::{Rect, Variant, Vector2};
use super::{BasicOperations, Operation, OperationFn};

fn operation_rect_with_f32(
    left: &Rect, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (left_min, left_max) = (left.min, left.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left_min.x, right), operation_fn_f32(left_min.y, right)
        ), 
        Vector2::new(
            operation_fn_f32(left_max.x, right), operation_fn_f32(left_max.y, right)
        )
    )
}

fn operation_rect_with_rect(
    left: &Rect, right: &Rect, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (left_min, left_max) = (left.min, left.max);
    let (right_min, right_max) = (right.min, right.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left_min.x, right_min.x), operation_fn_f32(left_min.y, right_min.y)
        ), 
        Vector2::new(
            operation_fn_f32(left_max.x, right_max.x), operation_fn_f32(left_max.y, right_max.y)
        )
    )
}

impl Operation for Rect {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Rect(with) => Some(Variant::Rect(operation_rect_with_rect(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Rect(operation_rect_with_f32(self, *with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Rect {}