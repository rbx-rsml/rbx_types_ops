use rbx_types::{Variant, Vector3, Vector3int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector3int16_with_f32(
    left: &Vector3int16, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left.x as f32, right) as i16,
        operation_fn_f32(left.y as f32, right) as i16,
        operation_fn_f32(left.z as f32, right) as i16,
    )
}

fn operation_vector3int16_with_vector3(
    left: &Vector3int16, right: &Vector3, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left.x as f32, right.x) as i16,
        operation_fn_f32(left.y as f32, right.y) as i16,
        operation_fn_f32(left.z as f32, right.z) as i16,
    )
}

fn operation_vector3int16_with_vector3int16(
    left: &Vector3int16, right: &Vector3int16, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left.x as f32, right.x as f32) as i16,
        operation_fn_f32(left.y as f32, right.y as f32) as i16,
        operation_fn_f32(left.z as f32, right.z as f32) as i16,
    )
}

impl Operation for Vector3int16 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Vector3int16(with) => Some(Variant::Vector3int16(operation_vector3int16_with_vector3int16(self, with, &operation_fn_f32))),
            Variant::Vector3(with) => Some(Variant::Vector3int16(operation_vector3int16_with_vector3(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector3int16(operation_vector3int16_with_f32(self, *with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector3int16 {}


// vec3int16 && vec3int16
#[test]
fn test_vec3int16_pow_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).pow(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3int16_div_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).div(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3int16_floor_div_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).floor_div(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3int16_modulus_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).modulus(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3int16_mult_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).mult(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3int16_add_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).add(&Vector3int16::new(5, 10, 5).into())
    )
}

#[test]
fn test_vec3int16_sub_vec3int16() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).sub(&Vector3int16::new(5, 10, 5).into())
    )
}

// vec3int16 && vec3
#[test]
fn test_vec3int16_pow_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).pow(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3int16_div_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).div(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3int16_floor_div_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).floor_div(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3int16_modulus_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).modulus(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3int16_mult_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).mult(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3int16_add_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).add(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

#[test]
fn test_vec3int16_sub_vec3() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).sub(&Vector3::new(5.0, 10.0, 5.0).into())
    )
}

// vec3int16 && f32
#[test]
fn test_vec3int16_pow_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).pow(&5f32.into())
    )
}

#[test]
fn test_vec3int16_div_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).div(&5f32.into())
    )
}

#[test]
fn test_vec3int16_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).floor_div(&5f32.into())
    )
}

#[test]
fn test_vec3int16_modulus_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).modulus(&5f32.into())
    )
}

#[test]
fn test_vec3int16_mult_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).mult(&5f32.into())
    )
}

#[test]
fn test_vec3int16_add_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).add(&5f32.into())
    )
}

#[test]
fn test_vec3int16_sub_f32() {
    insta::assert_yaml_snapshot!(
        Vector3int16::new(15, 15, 15).sub(&5f32.into())
    )
}