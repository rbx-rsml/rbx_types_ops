use num_traits::{Float, Num};
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

pub trait Operation {
    fn operation(
        self, variant: Variant,
        operation_fn_f32: OperationFn<f32>,
        operation_fn_i32: OperationFn<i32>
    ) -> Option<Variant>;
}

pub trait BasicOperations {
    fn pow(self, with: Variant) -> Option<Variant> 
    where
        Self: Sized + Operation 
    {
        self.operation(with, pow_f32, pow_i32)
    }

    fn div(self, with: Variant) -> Option<Variant>
    where
        Self: Sized + Operation 
    {
        self.operation(with, div, div)
    }

    fn floor_div(self, with: Variant) -> Option<Variant>
    where
        Self: Sized + Operation 
    {
        self.operation(with, floor_div, div)
    }

    fn modulo(self, with: Variant) -> Option<Variant>
    where
        Self: Sized + Operation
    {
        self.operation(with, modulus, modulus)
    }

    fn mult(self, with: Variant) -> Option<Variant>
    where
        Self: Sized + Operation
    {
        self.operation(with, mult, mult)
    }

    fn add(self, with: Variant) -> Option<Variant>
    where
        Self: Sized + Operation
    {
        self.operation(with, add, add)
    }

    fn sub(self, with: Variant) -> Option<Variant>
    where
        Self: Sized + Operation 
    {
        self.operation(with, sub, sub)
    }
}

macro_rules! match_basic_op {
    ($self:ident, $with:ident, $method:ident) => {
        match $self {
            Variant::Float32(left) => left.$method($with),
            Variant::UDim(left) => left.$method($with),
            Variant::UDim2(left) => left.$method($with),
            Variant::Vector3(left) => left.$method($with),
            Variant::Vector3int16(left) => left.$method($with),
            Variant::CFrame(left) => left.$method($with),
            Variant::Vector2(left) => left.$method($with),
            Variant::Vector2int16(left) => left.$method($with),
            Variant::Rect(left) => left.$method($with),
            Variant::Color3(left) => left.$method($with),
            _ => None
        }
    };
}

impl BasicOperations for Variant {
    fn pow(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, pow)
    }

    fn div(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, div)
    }

    fn floor_div(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, floor_div)
    }

    fn modulo(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, modulo)
    }

    fn mult(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, mult)
    }

    fn add(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, add)
    }

    fn sub(self, with: Variant) -> Option<Variant> {
        match_basic_op!(self, with, sub)
    }
}

fn will_divide_by_zero<T: Num>(a: T, b: T) -> bool {
    a.is_zero() || b.is_zero()
}

pub fn pow_f32(a: f32, b: f32) -> f32 { a.powf(b) }

pub fn pow_i32(a: i32, b: i32) -> i32 { a.pow(b as u32) }

pub fn div<T: Num + Copy>(a: T, b: T) -> T {
    if  a.is_zero() || b.is_zero() { return a }
    a / b
}

pub fn floor_div<T: Float>(a: T, b: T) -> T {
    if will_divide_by_zero(a, b) { return a }
    (a / b).floor()
}

pub fn modulus<T: Num>(a: T, b: T) -> T { a % b }

pub fn mult<T: Num>(a: T, b: T) -> T { a * b }

pub fn add<T: Num>(a: T, b: T) -> T { a + b }

pub fn sub<T: Num>(a: T, b: T) -> T { a - b }