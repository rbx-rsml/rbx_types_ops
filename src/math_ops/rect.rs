use rbx_types::{Rect, Variant, Vector2, Vector2int16};
use super::{BasicOperations, Operation, OperationFn};

fn operation_rect_with_f32(
    left: &Rect, right: f32, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (left_min, left_max) = (left.min, left.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left_min.x, right), operation_fn_f32(left_min.y, right)
        ), 
        Vector2::new(
            operation_fn_f32(left_max.x, right), operation_fn_f32(left_max.y, right)
        )
    )
}

fn operation_rect_with_rect(
    left: &Rect, right: &Rect, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (left_min, left_max) = (left.min, left.max);
    let (right_min, right_max) = (right.min, right.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left_min.x, right_min.x), operation_fn_f32(left_min.y, right_min.y)
        ), 
        Vector2::new(
            operation_fn_f32(left_max.x, right_max.x), operation_fn_f32(left_max.y, right_max.y)
        )
    )
}

fn operation_rect_with_vector2(
    left: &Rect, right: &Vector2, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (left_min, left_max) = (left.min, left.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left_min.x, right.x), operation_fn_f32(left_min.y, right.y)
        ), 
        Vector2::new(
            operation_fn_f32(left_max.x, right.x), operation_fn_f32(left_max.y, right.y)
        )
    )
}

fn operation_rect_with_vector2int16(
    left: &Rect, right: &Vector2int16, 
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (left_min, left_max) = (left.min, left.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left_min.x, right.x as f32), operation_fn_f32(left_min.y, right.y as f32)
        ), 
        Vector2::new(
            operation_fn_f32(left_max.x, right.x as f32), operation_fn_f32(left_max.y, right.y as f32)
        )
    )
}

impl Operation for Rect {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Rect(with) => Some(Variant::Rect(operation_rect_with_rect(self, with, &operation_fn_f32))),
            Variant::Float32(with) => Some(Variant::Rect(operation_rect_with_f32(self, *with, &operation_fn_f32))),
            Variant::Vector2(with) => Some(Variant::Rect(operation_rect_with_vector2(self, with, &operation_fn_f32))),
            Variant::Vector2int16(with) => Some(Variant::Rect(operation_rect_with_vector2int16(self, with, &operation_fn_f32))),
            _ => None
        }
    }
}



impl BasicOperations for Rect {}


// rect && rect
#[test]
fn test_rect_pow_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .pow(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}

#[test]
fn test_rect_div_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .div(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}

#[test]
fn test_rect_floor_div_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .floor_div(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}

#[test]
fn test_rect_modulus_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .modulus(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}

#[test]
fn test_rect_mult_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .mult(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}

#[test]
fn test_rect_add_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .add(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}

#[test]
fn test_rect_sub_rect() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .sub(&Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).into())
    )
}


// rect && f32
#[test]
fn test_rect_pow_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).pow(&4.0f32.into())
    )
}

#[test]
fn test_rect_div_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).div(&4.0f32.into())
    )
}

#[test]
fn test_rect_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).floor_div(&4.0f32.into())
    )
}

#[test]
fn test_rect_modulus_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).modulus(&4.0f32.into())
    )
}

#[test]
fn test_rect_mult_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).mult(&4.0f32.into())
    )
}

#[test]
fn test_rect_add_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).add(&4.0f32.into())
    )
}

#[test]
fn test_rect_sub_f32() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0)).sub(&4.0f32.into())
    )
}


// rect && vector2
#[test]
fn test_rect_pow_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .pow(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_rect_div_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .div(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_rect_floor_div_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .floor_div(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_rect_modulus_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .modulus(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_rect_mult_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .mult(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_rect_add_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .add(&Vector2::new(5.0, 10.0).into())
    )
}

#[test]
fn test_rect_sub_vector2() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .sub(&Vector2::new(5.0, 10.0).into())
    )
}

// rect && vector2int16
#[test]
fn test_rect_pow_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .pow(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_rect_div_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .div(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_rect_floor_div_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .floor_div(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_rect_modulus_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .modulus(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_rect_mult_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .mult(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_rect_add_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .add(&Vector2int16::new(5, 10).into())
    )
}

#[test]
fn test_rect_sub_vector2int16() {
    insta::assert_yaml_snapshot!(
        Rect::new(Vector2::new(15.0, 15.0), Vector2::new(50.0, 50.0))
            .sub(&Vector2int16::new(5, 10).into())
    )
}