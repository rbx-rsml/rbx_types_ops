use rbx_types::{Color3, Color3uint8, Variant};
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

fn operation_color3_with_color3u8(
    left: &Color3, right: &Color3uint8,
    operation_fn_f32: &OperationFn<f32>
) -> Color3 {
    Color3::new(
        operation_fn_f32(left.r, (right.r as f32) / 255.0),
        operation_fn_f32(left.g, (right.g as f32) / 255.0),
        operation_fn_f32(left.b, (right.b as f32) / 255.0)
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
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Color3(with) => Some(Variant::Color3(operation_color3_with_color3(self, with, &operation_fn_f32))),
            Variant::Color3uint8(with) => Some(Variant::Color3(operation_color3_with_color3u8(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Color3(operation_color3_with_f32(self, *with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Color3 {}

// color3 && f32
#[test]
fn test_color3_pow_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).pow(&5.0f32.into())
    )
}

#[test]
fn test_color3_div_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).div(&10.0f32.into())
    )
}

#[test]
fn test_color3_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).floor_div(&16.2f32.into())
    )
}

#[test]
fn test_color3_modulus_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).modulus(&16.2f32.into())
    )
}

#[test]
fn test_color3_mult_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).mult(&10.0f32.into())
    )
}

#[test]
fn test_color3_add_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).add(&10.0f32.into())
    )
}

#[test]
fn test_color3_sub_f32() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).sub(&5.0f32.into())
    )
}

// color3 && color3
#[test]
fn test_color3_pow_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).pow(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3_div_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).div(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3_floor_div_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).floor_div(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3_modulus_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).modulus(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3_mult_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).mult(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3_add_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).add(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3_sub_color3() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).sub(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

// color3 && color3uint8
#[test]
fn test_color3_pow_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).pow(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3_div_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).div(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3_floor_div_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).floor_div(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3_modulus_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).modulus(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3_mult_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).mult(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3_add_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).add(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3_sub_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3::new(1.0, 0.5, 0.2).sub(&Color3uint8::new(255, 128, 51).into())
    )
}