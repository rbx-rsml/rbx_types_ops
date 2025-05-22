use rbx_types::{CFrame, Matrix3, Variant, Vector3};
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
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::CFrame(with) => Some(Variant::CFrame(operation_cframe_with_cframe(&self, &with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for CFrame {}


// cframe && cframe
#[test]
fn test_cframe_pow_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).pow(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}

#[test]
fn test_cframe_div_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).div(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}

#[test]
fn test_cframe_floor_div_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).floor_div(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}

#[test]
fn test_cframe_modulus_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).modulus(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}

#[test]
fn test_cframe_mult_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).mult(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}

#[test]
fn test_cframe_add_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).add(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}

#[test]
fn test_cframe_sub_cframe() {
    insta::assert_yaml_snapshot!(
        CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).sub(&CFrame::new(Vector3::new(5.0, 10.0, 15.0), Matrix3::new(
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0),
            Vector3::new(6.0, 12.0, 18.0)
        )).into())
    )
}