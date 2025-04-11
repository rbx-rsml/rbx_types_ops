use rbx_types::{Color3, Color3uint8, Variant};
use super::{BasicOperations, Operation, OperationFn};

fn operation_color3u8_with_f32(
    left: &Color3uint8, right: f32,
    operation_fn_f32: &OperationFn<f32>,
) -> Color3uint8 {
    Color3uint8::new(
        operation_fn_f32(left.r as f32, right) as u8,
        operation_fn_f32(left.g as f32, right) as u8,
        operation_fn_f32(left.b as f32, right) as u8
    )
}

fn operation_color3u8_with_color3(
    left: &Color3uint8, right: &Color3,
    operation_fn_f32: &OperationFn<f32>
) -> Color3uint8 {
    Color3uint8::new(
        operation_fn_f32(left.r as f32, right.r) as u8,
        operation_fn_f32(left.g as f32, right.g) as u8,
        operation_fn_f32(left.b as f32, right.b) as u8
    )
}

fn operation_color3u8_with_color3u8(
    left: &Color3uint8, right: &Color3uint8,
    operation_fn_u8: &OperationFn<u8>,
) -> Color3uint8 {
    Color3uint8::new(
        operation_fn_u8(left.r, right.r),
        operation_fn_u8(left.g, right.g),
        operation_fn_u8(left.b, right.b)
    )
}

impl Operation for Color3uint8 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>,
        operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Color3uint8(with) => {
                Some(Variant::Color3uint8(operation_color3u8_with_color3u8(self, with, &operation_fn_u8)))
            },
            Variant::Color3(with) => {
                Some(Variant::Color3uint8(operation_color3u8_with_color3(self, with, &operation_fn_f32)))
            },
            Variant::Float32(with) => Some(Variant::Color3uint8(operation_color3u8_with_f32(self, *with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Color3uint8 {}