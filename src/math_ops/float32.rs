use rbx_types::{Color3, Rect, UDim, UDim2, Variant, Vector2, Vector2int16, Vector3, Vector3int16};
use super::{add_float, add_int, sub_float, sub_int, BasicOperations, Operation, OperationFn};

#[inline(always)]
fn operation_f32_with_f32(
    left: f32, right: f32,
    operation_fn_f32: &OperationFn<f32>
) -> f32 {
    operation_fn_f32(left, right)
}

#[inline(always)]
fn operation_f32_with_udim(
    left: f32, right: &UDim,
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim {
    UDim::new(operation_fn_f32(left, right.scale), operation_fn_i32(left as i32, right.offset))
}

#[inline(always)]
fn operation_f32_add_udim(
    left: f32, right: &UDim
) -> UDim {
    UDim::new(left + right.scale, right.offset)
}

#[inline(always)]
fn operation_f32_sub_udim(
    left: f32, right: &UDim
) -> UDim {
    UDim::new(left - right.scale, -right.offset)
}

#[inline(always)]
fn operation_f32_with_udim2(
    left: f32, right: &UDim2,
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim2 {
    UDim2::new(
        operation_f32_with_udim(left, &right.x, operation_fn_f32, operation_fn_i32), 
        operation_f32_with_udim(left, &right.y, operation_fn_f32, operation_fn_i32)
    )
}

#[inline(always)]
fn operation_f32_sub_udim2(
    left: f32, right: &UDim2
) -> UDim2 {
    UDim2::new(
        operation_f32_sub_udim(left, &right.x), 
        operation_f32_sub_udim(left, &right.y)
    )
}

#[inline(always)]
fn operation_f32_add_udim2(
    left: f32, right: &UDim2
) -> UDim2 {
    UDim2::new(
        operation_f32_add_udim(left, &right.x), 
        operation_f32_add_udim(left, &right.y)
    )
}

#[inline(always)]
fn operation_f32_with_vector3(
    left: f32, right: &Vector3,
    operation_fn_f32: &OperationFn<f32>
) -> Vector3 {
    Vector3::new(
        operation_fn_f32(left, right.x),
        operation_fn_f32(left, right.y),
        operation_fn_f32(left, right.z),
    )
}

#[inline(always)]
fn operation_f32_with_vector3int16(
    left: f32, right: &Vector3int16,
    operation_fn_f32: &OperationFn<f32>
) -> Vector3int16 {
    Vector3int16::new(
        operation_fn_f32(left, right.x as f32) as i16,
        operation_fn_f32(left, right.y as f32) as i16,
        operation_fn_f32(left, right.z as f32) as i16,
    )
}

#[inline(always)]
fn operation_f32_with_vector2(
    left: f32, right: &Vector2,
    operation_fn_f32: &OperationFn<f32>
) -> Vector2 {
    Vector2::new(
        operation_fn_f32(left, right.x),
        operation_fn_f32(left, right.y)
    )
}

#[inline(always)]
fn operation_f32_with_vector2int16(
    left: f32, right: &Vector2int16,
    operation_fn_f32: &OperationFn<f32>
) -> Vector2int16 {
    Vector2int16::new(
        operation_fn_f32(left, right.x as f32) as i16,
        operation_fn_f32(left, right.y as f32) as i16
    )
}

fn operation_f32_with_rect(
    left: f32, right: &Rect,
    operation_fn_f32: &OperationFn<f32>
) -> Rect {
    let (right_min, right_max) = (right.min, right.max);

    Rect::new(
        Vector2::new(
            operation_fn_f32(left, right_min.x), operation_fn_f32(left, right_min.y)
        ),
        Vector2::new(
            operation_fn_f32(left, right_max.x), operation_fn_f32(left, right_max.y)
        )
    )
}

fn operation_f32_with_color3(
    left: f32, right: &Color3,
    operation_fn_f32: &OperationFn<f32>
) -> Color3 {
    Color3::new(
        operation_fn_f32(left, right.r),
        operation_fn_f32(left, right.g),
        operation_fn_f32(left, right.b)
    )
}

impl Operation for f32 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Float32(with) => Some(Variant::Float32(operation_f32_with_f32(*self,*with, &operation_fn_f32))),
            Variant::UDim(with) => Some(Variant::UDim(operation_f32_with_udim(*self,with, &operation_fn_f32, &operation_fn_i32))),
            Variant::UDim2(with) => Some(Variant::UDim2(operation_f32_with_udim2(*self, with, &operation_fn_f32, &operation_fn_i32))),
            Variant::Vector3(with) => Some(Variant::Vector3(operation_f32_with_vector3(*self, with, &operation_fn_f32))),
            Variant::Vector3int16(with) => Some(Variant::Vector3int16(operation_f32_with_vector3int16(*self, with, &operation_fn_f32))),
            Variant::Vector2(with) => Some(Variant::Vector2(operation_f32_with_vector2(*self, with, &operation_fn_f32))),
            Variant::Vector2int16(with) => Some(Variant::Vector2int16(operation_f32_with_vector2int16(*self, with, &operation_fn_f32))),
            Variant::Rect(with) => Some(Variant::Rect(operation_f32_with_rect(*self, with, &operation_fn_f32))),
            Variant::Color3(with) => Some(Variant::Color3(operation_f32_with_color3(*self, with, &operation_fn_f32))),
            _ => None
        }
    }
}

impl BasicOperations for f32 {
    fn add(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::UDim(with) => Some(Variant::UDim(operation_f32_add_udim(*self, with))),
            Variant::UDim2(with) => Some(Variant::UDim2(operation_f32_add_udim2(*self, with))),
            _ => self.operation(with, add_float, add_int, add_int)
        }
    }

    fn sub(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::UDim(with) => Some(Variant::UDim(operation_f32_sub_udim(*self, with))),
            Variant::UDim2(with) => Some(Variant::UDim2(operation_f32_sub_udim2(*self, with))),
            _ => self.operation(with, sub_float, sub_int, sub_int)
        }
    }
}

// f32 && f32
#[test]
fn test_f32_pow_f32() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&10.0f32.into())
    )
}

#[test]
fn test_f32_div_f32() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&5.0f32.into())
    )
}

#[test]
fn test_f32_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&5.23f32.into())
    )
}

#[test]
fn test_f32_modulus_f32() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&15.0f32.into())
    )
}

#[test]
fn test_f32_mult_f32() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&5.0f32.into())
    )
}

#[test]
fn test_f32_add_f32() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&5.0f32.into())
    )
}

#[test]
fn test_f32_sub_f32() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&10.0f32.into())
    )
}

// f32 && udim
#[test]
fn test_f32_pow_udim() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&UDim::new(10.0, 32).into())
    )
}

#[test]
fn test_f32_div_udim() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&UDim::new(5.0, 32).into())
    )
}

#[test]
fn test_f32_floor_div_udim() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&UDim::new(5.23, 32).into())
    )
}

#[test]
fn test_f32_modulus_udim() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&UDim::new(15.0, 32).into())
    )
}

#[test]
fn test_f32_mult_udim() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&UDim::new(10.0, 32).into())
    )
}

#[test]
fn test_f32_add_udim() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&UDim::new(5.0, 32).into())
    )
}

#[test]
fn test_f32_sub_udim() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&UDim::new(10.0, 32).into())
    )
}

// f32 && udim2
#[test]
fn test_f32_pow_udim2() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&UDim2::new(
            UDim::new(2.0, 32),
            UDim::new(10.0, 32)
        ).into())
    )
}

#[test]
fn test_f32_div_udim2() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&UDim2::new(
            UDim::new(5.0, 32),
            UDim::new(5.0, 32)
        ).into())
    )
}

#[test]
fn test_f32_floor_div_udim2() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&UDim2::new(
            UDim::new(5.23, 32),
            UDim::new(5.23, 32)
        ).into())
    )
}

#[test]
fn test_f32_modulus_udim2() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&UDim2::new(
            UDim::new(15.0, 32),
            UDim::new(15.0, 32)
        ).into())
    )
}

#[test]
fn test_f32_mult_udim2() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&UDim2::new(
            UDim::new(10.0, 32),
            UDim::new(10.0, 32)
        ).into())
    )
}

#[test]
fn test_f32_add_udim2() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&UDim2::new(
            UDim::new(5.0, 32),
            UDim::new(5.0, 32)
        ).into())
    )
}

#[test]
fn test_f32_sub_udim2() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&UDim2::new(
            UDim::new(10.0, 32),
            UDim::new(10.0, 32)
        ).into())
    )
}


// f32 && vector3
#[test]
fn test_f32_pow_vector3() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

#[test]
fn test_f32_div_vector3() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

#[test]
fn test_f32_floor_div_vector3() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

#[test]
fn test_f32_modulus_vector3() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

#[test]
fn test_f32_mult_vector3() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

#[test]
fn test_f32_add_vector3() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

#[test]
fn test_f32_sub_vector3() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&Vector3::new(5.0, 10.0, 15.0).into())
    )
}

// f32 && vector3int16
#[test]
fn test_f32_pow_vector3int16() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&Vector3int16::new(5, 10, 15).into())
    )
}

#[test]
fn test_f32_div_vector3int16() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&Vector3int16::new(5, 10, 15).into())
    )
}

#[test]
fn test_f32_floor_div_vector3int16() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&Vector3int16::new(5, 10, 15).into())
    )
}

#[test]
fn test_f32_modulus_vector3int16() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&Vector3int16::new(5, 10, 15).into())
    )
}

#[test]
fn test_f32_mult_vector3int16() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&Vector3int16::new(5, 10, 15).into())
    )
}

#[test]
fn test_f32_add_vector3int16() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&Vector3int16::new(5, 10, 15).into())
    )
}

#[test]
fn test_f32_sub_vector3int16() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&Vector3int16::new(5, 10, 15).into())
    )
}

// f32 && vector2
#[test]
fn test_f32_pow_vector2() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&Vector2::new(5.0, 15.0).into())
    )
}

#[test]
fn test_f32_div_vector2() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&Vector2::new(5.0, 15.0).into())
    )
}

#[test]
fn test_f32_floor_div_vector2() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&Vector2::new(5.0, 15.0).into())
    )
}

#[test]
fn test_f32_modulus_vector2() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&Vector2::new(5.0, 15.0).into())
    )
}

#[test]
fn test_f32_mult_vector2() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&Vector2::new(5.0, 15.0).into())
    )
}

#[test]
fn test_f32_add_vector2() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&Vector2::new(5.0, 15.0).into())
    )
}

#[test]
fn test_f32_sub_vector2() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&Vector2::new(5.0, 15.0).into())
    )
}

// f32 && vector2int16
#[test]
fn test_f32_pow_vector2int16() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&Vector2int16::new(5, 15).into())
    )
}

#[test]
fn test_f32_div_vector2int16() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&Vector2int16::new(5, 15).into())
    )
}

#[test]
fn test_f32_floor_div_vector2int16() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&Vector2int16::new(5, 15).into())
    )
}

#[test]
fn test_f32_modulus_vector2int16() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&Vector2int16::new(5, 15).into())
    )
}

#[test]
fn test_f32_mult_vector2int16() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&Vector2int16::new(5, 15).into())
    )
}

#[test]
fn test_f32_add_vector2int16() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&Vector2int16::new(5, 15).into())
    )
}

#[test]
fn test_f32_sub_vector2int16() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&Vector2int16::new(5, 15).into())
    )
}

// f32 && rect
#[test]
fn test_f32_pow_rect() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

#[test]
fn test_f32_div_rect() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

#[test]
fn test_f32_floor_div_rect() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

#[test]
fn test_f32_modulus_rect() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

#[test]
fn test_f32_mult_rect() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

#[test]
fn test_f32_add_rect() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

#[test]
fn test_f32_sub_rect() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&Rect::new(
            Vector2::new(5.0, 10.0),
            Vector2::new(15.0, 20.0)
        ).into())
    )
}

// f32 && color3
#[test]
fn test_f32_pow_color3() {
    insta::assert_yaml_snapshot!(
        5.0f32.pow(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_f32_div_color3() {
    insta::assert_yaml_snapshot!(
        10.0f32.div(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_f32_floor_div_color3() {
    insta::assert_yaml_snapshot!(
        16.2f32.floor_div(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_f32_modulus_color3() {
    insta::assert_yaml_snapshot!(
        16.2f32.modulus(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_f32_mult_color3() {
    insta::assert_yaml_snapshot!(
        10.0f32.mult(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_f32_add_color3() {
    insta::assert_yaml_snapshot!(
        10.0f32.add(&Color3::new(1.0, 0.5, 0.2).into())
    )
}

#[test]
fn test_f32_sub_color3() {
    insta::assert_yaml_snapshot!(
        5.0f32.sub(&Color3::new(1.0, 0.5, 0.2).into())
    )
}