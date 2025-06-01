use rbx_types::{Rect, Variant, Vector2, Vector2int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_vector2_with_f32(
    left: &Vector2, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left.x, right),
        operation_fn_f32(left.y, right)
    )
}

fn operation_vector2_with_vector2int16(
    left: &Vector2, right: &Vector2int16, 
    operation_fn_f32: &OperationFn<f32>,
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left.x, right.x as f32),
        operation_fn_f32(left.y, right.y as f32)
    )
}

fn operation_vector2_with_vector2(
    left: &Vector2, right: &Vector2, 
    operation_fn_f32: &OperationFn<f32>,
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left.x, right.x),
        operation_fn_f32(left.y, right.y)
    )
}

fn operation_vector2_with_rect(
    left: &Vector2, right: &Rect, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (right_min, right_max) = (right.min, right.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left.x, right_min.x), operation_fn_f32(left.y, right_min.y)
        ), 
        Vector2::new(
            operation_fn_f32(left.x, right_max.x), operation_fn_f32(left.y, right_max.y)
        )
    )
}

impl Operation for Vector2 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Vector2(with) => Some(Variant::Vector2(operation_vector2_with_vector2(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Vector2(operation_vector2_with_f32(self, *with, &operation_fn_f32))),
            Variant::Vector2int16(with) => Some(Variant::Vector2(operation_vector2_with_vector2int16(self, with, &operation_fn_f32))),
            Variant::Rect(with) => Some(Variant::Rect(operation_vector2_with_rect(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for Vector2 {}

// vec2 && vec2
#[test]
fn test_vec2_pow_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).pow(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_vec2_div_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).div(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_vec2_floor_div_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).floor_div(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_vec2_modulus_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).modulus(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_vec2_mult_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).mult(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_vec2_add_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).add(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_vec2_sub_vec2() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).sub(&Vector2::new(5.0, 10.0).into())
    )
}

// vec2 && vec2int16
#[test]
fn test_vec2_pow_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).pow(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_vec2_div_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).div(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_vec2_floor_div_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).floor_div(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_vec2_modulus_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).modulus(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_vec2_mult_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).mult(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_vec2_add_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).add(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_vec2_sub_vec2int16() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).sub(&Vector2int16::new(5, 10).into())
    )
}

// vec2 && f32
#[test]
fn test_vec2_pow_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).pow(&5f32.into())
    )
}

#[test]
fn test_vec2_div_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).div(&5f32.into())
    )
}

#[test]
fn test_vec2_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).floor_div(&5f32.into())
    )
}

#[test]
fn test_vec2_modulus_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).modulus(&5f32.into())
    )
}

#[test]
fn test_vec2_mult_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).mult(&5f32.into())
    )
}

#[test]
fn test_vec2_add_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).add(&5f32.into())
    )
}

#[test]
fn test_vec2_sub_f32() {
    insta::assert_yaml_snapshot!(
        Vector2::new(15.0, 15.0).sub(&5f32.into())
    )
}

// vec2 && rect
#[test]
fn test_vec2_pow_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).pow(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}

#[test]
fn test_vec2_div_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).div(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}

#[test]
fn test_vec2_floor_div_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).floor_div(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}

#[test]
fn test_vec2_modulus_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).modulus(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}

#[test]
fn test_vec2_mult_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).mult(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}

#[test]
fn test_vec2_add_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).add(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}

#[test]
fn test_vec2_sub_rect() {
    insta::assert_yaml_snapshot!(
        Vector2::new(175.0, 5.0).sub(&Rect::new(Vector2::new(5.0, 10.0), Vector2::new(5.0, 10.0)).into())
    )
}