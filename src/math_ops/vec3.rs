use rbx_types::{Variant, Vector3, Vector3int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector3_with_f32(
    left: &Vector3, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left.x, right),
        operation_fn_f32(left.y, right),
        operation_fn_f32(left.z, right),
    )
}

fn operation_vector3_with_vector3int16(
    left: &Vector3, right: &Vector3int16, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left.x, right.x as f32),
        operation_fn_f32(left.y, right.y as f32),
        operation_fn_f32(left.z, right.z as f32),
    )
}

pub fn operation_vector3_with_vector3(
    left: &Vector3, right: &Vector3, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left.x, right.x),
        operation_fn_f32(left.y, right.y),
        operation_fn_f32(left.z, right.z),
    )
}

impl Operation for Vector3 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Vector3(with) => Some(Variant::Vector3(operation_vector3_with_vector3(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector3(operation_vector3_with_f32(self, *with, &operation_fn_f32))),
            Variant::Vector3int16(with) => Some(Variant::Vector3(operation_vector3_with_vector3int16(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector3 {}


// vec3 && vec3
#[test]
fn test_vec3_pow_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).pow(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3_div_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).div(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3_floor_div_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).floor_div(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3_modulus_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).modulus(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3_mult_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).mult(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3_add_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).add(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3_sub_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).sub(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

// vec3 && vec3int16
#[test]
fn test_vec3_pow_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).pow(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3_div_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).div(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3_floor_div_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).floor_div(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3_modulus_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).modulus(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3_mult_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).mult(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3_add_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).add(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3_sub_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).sub(&Vector3int16::new(5, 10, 5).into())
    )
}

// vec3 && f32
#[test]
fn test_vec3_pow_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).pow(&5f32.into())
    )
}

#[test]
fn test_vec3_div_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).div(&5f32.into())
    )
}

#[test]
fn test_vec3_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).floor_div(&5f32.into())
    )
}

#[test]
fn test_vec3_modulus_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).modulus(&5f32.into())
    )
}

#[test]
fn test_vec3_mult_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).mult(&5f32.into())
    )
}

#[test]
fn test_vec3_add_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).add(&5f32.into())
    )
}

#[test]
fn test_vec3_sub_f32() {
    insta::assert_yaml_snapshot!(
        Vector3::new(15.0, 15.0, 15.0).sub(&5f32.into())
    )
}
