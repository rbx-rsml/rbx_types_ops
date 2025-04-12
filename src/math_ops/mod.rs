use num_traits::{Float, Num, PrimInt};
use rbx_types::Variant;

pub type OperationFn<N> = fn(N, N) -> N;

mod float32;
mod udim;
mod udim2;
mod vec3;
mod vec3i16;
mod cframe;
mod vec2;
mod vec2i16;
mod rect;
mod color3;
mod color3u8;

pub trait Operation {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>,
        operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant>;
}

pub trait BasicOperations where Self: Sized {
    fn pow(&self, with: &Variant) -> Option<Variant> 
    where
        Self: Sized + Operation 
    {
        self.operation(with, pow_float, pow_int, pow_int)
    }

    fn div(&self, with: &Variant) -> Option<Variant>
    where
        Self: Sized + Operation 
    {
        self.operation(with, div, div, div)
    }

    fn floor_div(&self, with: &Variant) -> Option<Variant>
    where
        Self: Sized + Operation 
    {
        self.operation(with, floor_div, div, div)
    }

    fn modulus(&self, with: &Variant) -> Option<Variant>
    where
        Self: Sized + Operation
    {
        self.operation(with, modulus, modulus, modulus)
    }

    fn mult(&self, with: &Variant) -> Option<Variant>
    where
        Self: Sized + Operation
    {
        self.operation(with, mult_float, mult_int, mult_int)
    }

    fn add(&self, with: &Variant) -> Option<Variant>
    where
        Self: Sized + Operation
    {
        self.operation(with, add_float, add_int, add_int)
    }

    fn sub(&self, with: &Variant) -> Option<Variant>
    where
        Self: Sized + Operation 
    {   
        self.operation(with, sub_float, sub_int, sub_int)
    }
}

macro_rules! match_basic_op {
    ($self:ident, $with:ident, $method:ident) => {
        match $self {
            Variant::Float32(left) => left.$method(&$with),
            Variant::UDim(left) => left.$method(&$with),
            Variant::UDim2(left) => left.$method(&$with),
            Variant::Vector3(left) => left.$method(&$with),
            Variant::Vector3int16(left) => left.$method(&$with),
            Variant::CFrame(left) => left.$method(&$with),
            Variant::Vector2(left) => left.$method(&$with),
            Variant::Vector2int16(left) => left.$method(&$with),
            Variant::Rect(left) => left.$method(&$with),
            Variant::Color3(left) => left.$method(&$with),
            _ => None
        }
    };
}

impl BasicOperations for Variant where Self: Sized {
    fn pow(self: &Variant, with: &Variant) -> Option<Variant>  {
        match_basic_op!(self, with, pow)
    }

    fn div(self: &Variant, with: &Variant) -> Option<Variant> {
        match_basic_op!(self, with, div)
    }

    fn floor_div(self: &Variant, with: &Variant) -> Option<Variant> {
        match_basic_op!(self, with, floor_div)
    }

    fn modulus(self: &Variant, with: &Variant) -> Option<Variant> {
        match_basic_op!(self, with, modulus)
    }

    fn mult(self: &Variant, with: &Variant) -> Option<Variant> {
        match_basic_op!(self, with, mult)
    }

    fn add(self: &Variant, with: &Variant) -> Option<Variant> {
        match_basic_op!(self, with, add)
    }

    fn sub(self: &Variant, with: &Variant) -> Option<Variant> {
        match_basic_op!(self, with, sub)
    }
}

impl Operation for Variant {
    fn operation(
        &self, with: &Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>,
        operation_fn_u8: OperationFn<u8>
    ) -> Option<Variant> { 
        match self {
            Variant::Float32(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::UDim(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::UDim2(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Vector3(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Vector3int16(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::CFrame(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Vector2(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Vector2int16(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Rect(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Color3(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            Variant::Color3uint8(left) => left.operation(with, operation_fn_f32, operation_fn_i32, operation_fn_u8),
            _ => None
        }
    }
}

pub fn pow_float<N: Float>(a: N, b: N) -> N { a.powf(b) }
pub fn pow_int<N: PrimInt>(a: N, b: N) -> N { a.pow(b.to_u32().unwrap_or(u32::MAX)) }

pub fn div<T: Num + Copy>(a: T, b: T) -> T { if a.is_zero() || b.is_zero() { a } else { a / b } }
pub fn floor_div<T: Float>(a: T, b: T) -> T { if a.is_zero() || b.is_zero() { a } else { (a / b).floor() } }

pub fn modulus<T: Num>(a: T, b: T) -> T { if b.is_zero() { a } else { a % b } }

pub fn mult_float<T: Float>(a: T, b: T) -> T { a.mul(b) }
pub fn mult_int<T: PrimInt>(a: T, b: T) -> T { a.checked_mul(&b).unwrap_or(T::max_value()) }

pub fn add_float<T: Float>(a: T, b: T) -> T { a.add(b) }
pub fn add_int<T: PrimInt>(a: T, b: T) -> T { a.checked_add(&b).unwrap_or(T::max_value()) }

pub fn sub_float<T: Float>(a: T, b: T) -> T { a.sub(b) }
pub fn sub_int<T: PrimInt>(a: T, b: T) -> T { a.checked_sub(&b).unwrap_or(T::min_value()) }