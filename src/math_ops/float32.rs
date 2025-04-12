use rbx_types::{Color3, Rect, UDim, UDim2, Variant, Vector2, Vector2int16, Vector3, Vector3int16};
use super::{sub_float, sub_int, BasicOperations, Operation, OperationFn};

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
    operation_fn_f32: &OperationFn<f32>
) -> UDim {
    UDim::new(operation_fn_f32(left, right.scale), right.offset)
}

#[inline(always)]
fn operation_sub_f32_with_udim(
    left: f32, right: &UDim
) -> UDim {
    UDim::new(left - right.scale, -right.offset)
}

#[inline(always)]
fn operation_f32_with_udim2(
    left: f32, right: &UDim2,
    operation_fn_f32: &OperationFn<f32>
) -> UDim2 {
    UDim2::new(
        operation_f32_with_udim(left, &right.x, operation_fn_f32), 
        operation_f32_with_udim(left, &right.y, operation_fn_f32)
    )
}

#[inline(always)]
fn operation_sub_f32_with_udim2(
    left: f32, right: &UDim2
) -> UDim2 {
    UDim2::new(
        operation_sub_f32_with_udim(left, &right.x), 
        operation_sub_f32_with_udim(left, &right.y)
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
        _operation_fn_i32: OperationFn<i32>,
        _operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> {
        match with {
            Variant::Float32(with) => Some(Variant::Float32(operation_f32_with_f32(*self,*with, &operation_fn_f32))),
            Variant::UDim(with) => Some(Variant::UDim(operation_f32_with_udim(*self,with, &operation_fn_f32))),
            Variant::UDim2(with) => Some(Variant::UDim2(operation_f32_with_udim2(*self, with, &operation_fn_f32))),
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
    fn sub(&self, with: &Variant) -> Option<Variant> {
        match with {
            Variant::UDim(with) => Some(Variant::UDim(operation_sub_f32_with_udim(*self, with))),
            Variant::UDim2(with) => Some(Variant::UDim2(operation_sub_f32_with_udim2(*self, with))),
            _ => self.operation(with, sub_float, sub_int, sub_int)
        }
    }
}