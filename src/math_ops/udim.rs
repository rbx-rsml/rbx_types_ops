use rbx_types::{UDim, Variant};
use super::{add_float, add_int, sub_float, sub_int, BasicOperations, Operation, OperationFn};

fn operation_udim_with_f32(
    left: &UDim, right: f32, 
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim {
    UDim::new(operation_fn_f32(left.scale, right), operation_fn_i32(left.offset, right as i32))
}

fn operation_udim_add_f32(
    left: &UDim, right: f32
) -> UDim {
    UDim::new(left.scale + right, left.offset)
}


fn operation_udim_sub_f32(
    left: &UDim, right: f32
) -> UDim {
    UDim::new(left.scale - right, left.offset)
}

pub fn operation_udim_with_udim(
    left: &UDim, right: &UDim, 
    operation_fn_f32: &OperationFn<f32>,
    operation_fn_i32: &OperationFn<i32>
) -> UDim {
    UDim::new(
        operation_fn_f32(left.scale, right.scale), 
        operation_fn_i32(left.offset, right.offset)
    )
}

impl Operation for UDim {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::UDim(with) => Some(Variant::UDim(operation_udim_with_udim(self, with, &operation_fn_f32, &operation_fn_i32))),
            Variant::Float32(with) => Some(Variant::UDim(operation_udim_with_f32(self, *with, &operation_fn_f32, &operation_fn_i32))),
            _ => None
        }
    }
}

impl BasicOperations for UDim {
    fn add(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::Float32(with) => Some(Variant::UDim(operation_udim_add_f32(self, *with))),
            _ => self.operation(with, add_float, add_int, add_int)
        }
    }

    fn sub(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::Float32(with) => Some(Variant::UDim(operation_udim_sub_f32(self, *with))),
            _ => self.operation(with, sub_float, sub_int, sub_int)
        }
    }
}

// udim && f32
#[test]
fn test_udim_pow_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(10.0, 32).pow(&5.0f32.into())
    )
}

#[test]
fn test_udim_div_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(5.0, 32).div(&10.0f32.into())
    )
}

#[test]
fn test_udim_floor_div_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(5.23, 32).floor_div(&16.2f32.into())
    )
}

#[test]
fn test_udim_modulus_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(15.0, 32).modulus(&16.2f32.into())
    )
}

#[test]
fn test_udim_mult_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(10.0, 32).mult(&10.0f32.into())
    )
}

#[test]
fn test_udim_add_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(5.0, 32).add(&10.0f32.into())
    )
}

#[test]
fn test_udim_sub_f32() {
    insta::assert_yaml_snapshot!(
        UDim::new(10.0, 32).sub(&5.0f32.into())
    )
}

// udim && udim
#[test]
fn test_udim_pow_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(10.0, 16).pow(&UDim::new(5.0, 32).into())
    )
}

#[test]
fn test_udim_div_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(5.0, 16).div(&UDim::new(10.0, 32).into())
    )
}

#[test]
fn test_udim_floor_div_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(5.23, 16).floor_div(&UDim::new(16.2, 32).into())
    )
}

#[test]
fn test_udim_modulus_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(15.0, 16).modulus(&UDim::new(16.2, 32).into())
    )
}

#[test]
fn test_udim_mult_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(10.0, 16).mult(&UDim::new(10.0, 32).into())
    )
}

#[test]
fn test_udim_add_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(5.0, 16).add(&UDim::new(10.0, 32).into())
    )
}

#[test]
fn test_udim_sub_udim() {
    insta::assert_yaml_snapshot!(
        UDim::new(10.0, 16).sub(&UDim::new(5.0, 32).into())
    )
}