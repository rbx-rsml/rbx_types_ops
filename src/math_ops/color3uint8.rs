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
        operation_fn_f32(left.r as f32, right.r * 255.0) as u8,
        operation_fn_f32(left.g as f32, right.g * 255.0) as u8,
        operation_fn_f32(left.b as f32, right.b * 255.0) as u8
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


// color3uint8 && f32
#[test]
fn test_color3uint8_pow_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).pow(&5.0f32.into())
    )
}

#[test]
fn test_color3uint8_div_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).div(&10.0f32.into())
    )
}

#[test]
fn test_color3uint8_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).floor_div(&16.2f32.into())
    )
}

#[test]
fn test_color3uint8_modulus_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).modulus(&16.2f32.into())
    )
}

#[test]
fn test_color3uint8_mult_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).mult(&10.0f32.into())
    )
}

#[test]
fn test_color3uint8_add_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).add(&10.0f32.into())
    )
}

#[test]
fn test_color3uint8_sub_f32() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).sub(&5.0f32.into())
    )
}

// color3uint8 && color3uint8
#[test]
fn test_color3uint8_pow_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).pow(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3uint8_div_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).div(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3uint8_floor_div_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).floor_div(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3uint8_modulus_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).modulus(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3uint8_mult_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).mult(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3uint8_add_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).add(&Color3uint8::new(255, 128, 51).into())
    )
}

#[test]
fn test_color3uint8_sub_color3uint8() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).sub(&Color3uint8::new(255, 128, 51).into())
    )
}

// color3uint8 && color3
#[test]
fn test_color3uint8_pow_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).pow(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3uint8_div_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).div(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3uint8_floor_div_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).floor_div(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3uint8_modulus_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).modulus(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3uint8_mult_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).mult(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3uint8_add_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).add(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_color3uint8_sub_color3() {
    insta::assert_yaml_snapshot!(
        Color3uint8::new(255, 128, 51).sub(&Color3::new(1.0, 0.5, 0.2).into())
    )
}