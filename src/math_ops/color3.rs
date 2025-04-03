use rbx_types::{Color3, Variant};
use super::{BasicOperations, Operation, OperationFn};

fn operation_color3_with_f32(
    left: &Color3, right: f32,
    operation_fn_f32: &OperationFn<f32>,
) -> Color3 {
    Color3::new(
        operation_fn_f32(left.r, right),
        operation_fn_f32(left.g, right),
        operation_fn_f32(left.b, right)
    )
}


fn operation_color3_with_color3(
    left: &Color3, right: &Color3,
    operation_fn_f32: &OperationFn<f32>
) -> Color3 {
    Color3::new(
        operation_fn_f32(left.r, right.r),
        operation_fn_f32(left.g, right.g),
        operation_fn_f32(left.b, right.b)
    )
}

impl Operation for Color3 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::Color3(with) => Some(Variant::Color3(operation_color3_with_color3(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Color3(operation_color3_with_f32(self, *with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Color3 {}