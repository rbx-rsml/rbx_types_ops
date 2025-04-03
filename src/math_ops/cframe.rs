use rbx_types::{CFrame, Matrix3, Variant};
use super::{BasicOperations, Operation, OperationFn};

use super::vec3::operation_vector3_with_vector3;

fn operation_cframe_with_cframe(
    left: &CFrame, right: &CFrame, 
    operation_fn_f32: &OperationFn<f32>
) -> CFrame {
    let left_orien = left.orientation;
    let right_orien = right.orientation;

    CFrame::new(
        operation_vector3_with_vector3(&left.position, &right.position, operation_fn_f32),
        Matrix3::new(
            operation_vector3_with_vector3(&left_orien.x, &right_orien.x, operation_fn_f32),
            operation_vector3_with_vector3(&left_orien.y, &right_orien.y, operation_fn_f32),
            operation_vector3_with_vector3(&left_orien.z, &right_orien.z, operation_fn_f32)
        )
    )
}

impl Operation for CFrame {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant> {
        match with {
            Variant::CFrame(with) => Some(Variant::CFrame(operation_cframe_with_cframe(&self, &with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for CFrame {}