use rbx_types::{UDim, UDim2, Variant};
use super::{add_float, add_int, sub_float, sub_int, udim::operation_udim_with_udim, BasicOperations, Operation, OperationFn};

fn operation_udim2_with_f32(
    left: &UDim2, right: f32,
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim2 {
    let (left_x, left_y) = (left.x, left.y);

    UDim2::new(
        UDim::new(
            operation_fn_f32(left_x.scale, right),
            operation_fn_i32(left_x.offset, right as i32)
        ),
        UDim::new(
            operation_fn_f32(left_y.scale, right),
            operation_fn_i32(left_x.offset, right as i32)
        )
    )
}

fn operation_udim2_add_f32(
    left: &UDim2, right: f32
) -> UDim2 {
    let (left_x, left_y) = (left.x, left.y);

    UDim2::new(
        UDim::new(left_x.scale + right, left_x.offset),
        UDim::new(left_y.scale + right, left_y.offset),
    )
}

fn operation_udim2_sub_f32(
    left: &UDim2, right: f32
) -> UDim2 {
    let (left_x, left_y) = (left.x, left.y);

    UDim2::new(
        UDim::new(left_x.scale - right, left_x.offset),
        UDim::new(left_y.scale - right, left_y.offset),
    )
}

fn operation_udim2_with_udim2(
    left: &UDim2, right: &UDim2, 
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim2 {
    let (left_x, left_y) = (left.x, left.y);
    let (right_x, right_y) = (right.x, right.y);

    UDim2::new(
        operation_udim_with_udim(&left_x, &right_x, operation_fn_f32, operation_fn_i32),
        operation_udim_with_udim(&left_y, &right_y, operation_fn_f32, operation_fn_i32),
    )
}

impl Operation for UDim2 {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {

        match with {
            Variant::UDim2(with) => Some(Variant::UDim2(operation_udim2_with_udim2(self, with, &operation_fn_f32, &operation_fn_i32))),
            Variant::Float32(with) => Some(Variant::UDim2(operation_udim2_with_f32(self, *with, &operation_fn_f32, &operation_fn_i32))),
            _ => None
        }
    }
}

impl BasicOperations for UDim2 {
    fn add(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::Float32(with) => Some(Variant::UDim2(operation_udim2_add_f32(self, *with))),
            _ => self.operation(with, add_float, add_int, add_int)
        }
    }

    fn sub(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::Float32(with) => Some(Variant::UDim2(operation_udim2_sub_f32(self, *with))),
            _ => self.operation(with, sub_float, sub_int, sub_int)
        }
    }
}


// udim2 && f32
#[test]
fn test_udim2_pow_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32)).pow(&5.0f32.into())
    )
}

#[test]
fn test_udim2_div_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(5.0, 32), UDim::new(5.0, 32)).div(&10.0f32.into())
    )
}

#[test]
fn test_udim2_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(5.23, 32), UDim::new(5.23, 32)).floor_div(&16.2f32.into())
    )
}

#[test]
fn test_udim2_modulus_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(15.0, 32), UDim::new(15.0, 32)).modulus(&16.2f32.into())
    )
}

#[test]
fn test_udim2_mult_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32)).mult(&10.0f32.into())
    )
}

#[test]
fn test_udim2_add_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(5.0, 32), UDim::new(5.0, 32)).add(&10.0f32.into())
    )
}

#[test]
fn test_udim2_sub_f32() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32)).sub(&5.0f32.into())
    )
}

// udim2 && udim2
#[test]
fn test_udim2_pow_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32))
            .pow(&UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32)).into())
    )
}

#[test]
fn test_udim2_div_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(5.0, 32), UDim::new(5.0, 32))
            .div(&UDim2::new(UDim::new(5.0, 32), UDim::new(5.0, 32)).into())
    )
}

#[test]
fn test_udim2_floor_div_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(5.23, 32), UDim::new(5.23, 32))
            .floor_div(&UDim2::new(UDim::new(5.23, 32), UDim::new(5.23, 32)).into())
    )
}

#[test]
fn test_udim2_modulus_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(15.0, 32), UDim::new(15.0, 32))
            .modulus(&UDim2::new(UDim::new(15.0, 32), UDim::new(15.0, 32)).into())
    )
}

#[test]
fn test_udim2_mult_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32))
            .mult(&UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32)).into())
    )
}

#[test]
fn test_udim2_add_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(5.0, 32), UDim::new(5.0, 32))
            .add(&UDim2::new(UDim::new(5.0, 32), UDim::new(5.0, 32)).into())
    )
}

#[test]
fn test_udim2_sub_udim2() {
    insta::assert_yaml_snapshot!(
        UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32))
            .sub(&UDim2::new(UDim::new(10.0, 32), UDim::new(10.0, 32)).into())
    )
}