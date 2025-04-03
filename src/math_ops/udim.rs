use rbx_types::{UDim, Variant};
use super::{BasicOperations, Operation, OperationFn};

fn operation_udim_with_f32(
    left: UDim, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> UDim {
    UDim::new(operation_fn_f32(left.scale, right), left.offset)
}

fn operation_udim_with_udim(
    left: UDim, right: UDim, 
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim {
    UDim::new(
        operation_fn_f32(left.scale, right.scale), 
        operation_fn_i32(left.offset, right.offset)
    )
}

impl Operation for UDim {
    fn operation(
        self, with: Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::UDim(with) => Some(Variant::UDim(operation_udim_with_udim(self, with, &operation_fn_f32, &operation_fn_i32))),
            Variant::Float32(with) => Some(Variant::UDim(operation_udim_with_f32(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for UDim {}