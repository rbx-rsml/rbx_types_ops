use rbx_types::{UDim, UDim2, Variant};
use super::{BasicOperations, Operation, OperationFn};

fn operation_udim2_with_f32(
    left: UDim2, right: f32,
    operation_fn_f32: &OperationFn<f32>
) -> UDim2 {
    let (left_x, left_y) = (left.x, left.y);

    UDim2::new(
        UDim::new(operation_fn_f32(left_x.scale, right), left_x.offset),
        UDim::new(operation_fn_f32(left_y.scale, right), left_y.offset)
    )
}

fn operation_udim2_with_udim2(
    left: UDim2, right: UDim2, 
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim2 {
    let (left_x, left_y) = (left.x, left.y);
    let (right_x, right_y) = (right.x, right.y);

    UDim2::new(
        UDim::new(
            operation_fn_f32(left_x.scale, right_x.scale), 
            operation_fn_i32(left_x.offset, right_x.offset)
        ),
        UDim::new(
            operation_fn_f32(left_y.scale, right_y.scale), 
            operation_fn_i32(left_y.offset, right_y.offset)
        )
    )
}

impl Operation for UDim2 {
    fn operation(
        self, with: Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::UDim2(with) => Some(Variant::UDim2(operation_udim2_with_udim2(self, with, &operation_fn_f32, &operation_fn_i32))),
            Variant::Float32(with) => Some(Variant::UDim2(operation_udim2_with_f32(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for UDim2 {}