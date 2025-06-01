use num_traits::{Float, PrimInt};
use rbx_types::{CFrame, Color3, Color3uint8, Matrix3, Rect, UDim, UDim2, Vector2, Vector2int16, Vector3, Vector3int16, Variant};

macro_rules! implement_method_for_datatypes {
    ($trait_name:ident, $method_name:ident) => {
        impl $trait_name for f32 {
            fn $method_name(self) -> Self {
                Float::$method_name(self)
            }
        }

        impl $trait_name for UDim {
            fn $method_name(self) -> Self {
                UDim::new(
                    self.scale.$method_name(),
                    self.offset.$method_name()
                )
            }
        }
        
        impl $trait_name for UDim2 {
            fn $method_name(self) -> Self {
                UDim2::new(
                    self.x.$method_name(),
                    self.y.$method_name()
                )
            }
        }
        
        impl $trait_name for Rect {
            fn $method_name(self) -> Self {
                Rect::new(
                    self.min.$method_name(),
                    self.max.$method_name(),
                )
            }
        }
        
        impl $trait_name for Vector2 {
            fn $method_name(self) -> Self {
                Vector2::new(
                    self.x.$method_name(),
                    self.y.$method_name()
                )
            }
        }

        impl $trait_name for Vector2int16 {
            fn $method_name(self) -> Self {
                Vector2int16::new(
                    self.x.$method_name(),
                    self.y.$method_name()
                )
            }
        }
        
        impl $trait_name for Vector3 {
            fn $method_name(self) -> Self {
                Vector3::new(
                    self.x.$method_name(),
                    self.y.$method_name(),
                    self.z.$method_name()
                )
            }
        }

        impl $trait_name for Vector3int16 {
            fn $method_name(self) -> Self {
                Vector3int16::new(
                    self.x.$method_name(),
                    self.y.$method_name(),
                    self.z.$method_name()
                )
            }
        }
        
        impl $trait_name for CFrame {
            fn $method_name(self) -> Self {
                CFrame::new(
                    self.position.$method_name(),
                    self.orientation.$method_name()
                )
            }
        }

        impl $trait_name for Matrix3 {
            fn $method_name(self) -> Self {
                Matrix3::new(
                    self.x.$method_name(),
                    self.y.$method_name(),
                    self.z.$method_name()
                )
            }
        }
        
        impl $trait_name for Color3 {
            fn $method_name(self) -> Self {
                Color3::new(
                    self.r.$method_name(),
                    self.g.$method_name(),
                    self.b.$method_name(),
                )
            }
        }

        impl $trait_name for Color3uint8 {
            fn $method_name(self) -> Self {
                Color3uint8::new(
                    self.r.$method_name(),
                    self.g.$method_name(),
                    self.b.$method_name(),
                )
            }
        }

        impl $trait_name for Variant {
            fn $method_name(self) -> Self {
                match self {
                    Variant::Float32(inner) => Variant::Float32(inner.$method_name()),
                    Variant::UDim(inner) => Variant::UDim(inner.$method_name()),
                    Variant::UDim2(inner) => Variant::UDim2(inner.$method_name()),
                    Variant::Vector3(inner) => Variant::Vector3(inner.$method_name()),
                    Variant::Vector3int16(inner) => Variant::Vector3int16(inner.$method_name()),
                    Variant::CFrame(inner) => Variant::CFrame(inner.$method_name()),
                    Variant::Vector2(inner) => Variant::Vector2(inner.$method_name()),
                    Variant::Vector2int16(inner) => Variant::Vector2int16(inner.$method_name()),
                    Variant::Rect(inner) => Variant::Rect(inner.$method_name()),
                    Variant::Color3(inner) => Variant::Color3(inner.$method_name()),
                    Variant::Color3uint8(inner) =>  Variant::Color3uint8(inner.$method_name()),
                    _ => self
                }
            }
        }
    };
}

pub trait Ceil {
    fn ceil(self) -> Self;
}
implement_method_for_datatypes!(Ceil, ceil);

pub trait Floor {
    fn floor(self) -> Self;
}
implement_method_for_datatypes!(Floor, floor);

pub trait Round {
    fn round(self) -> Self;
}
implement_method_for_datatypes!(Round, round);

pub trait Abs {
    fn abs(self) -> Self;
}
implement_method_for_datatypes!(Abs, abs);


trait UnsignedAbs {
    fn abs(self) -> Self where Self: PrimInt {
        self
    }
}

trait IntFloorCeilRound {
    fn floor(self) -> Self where Self: PrimInt {
        self
    }
    fn ceil(self) -> Self where Self: PrimInt {
        self
    }
    fn round(self) -> Self where Self: PrimInt {
        self
    }
}

impl UnsignedAbs for u8 {}
impl IntFloorCeilRound for u8 {}

impl IntFloorCeilRound for i32 {}
impl IntFloorCeilRound for i16 {}


// floor
#[test]
fn test_f32_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(5.3f32).floor()
    )
}

#[test]
fn test_udim_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim::new(10.5, 15)).floor()
    )
}

#[test]
fn test_udim2_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim2::new(UDim::new(10.5, 15), UDim::new(10.5, 15))).floor()
    )
}

#[test]
fn test_rect_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Rect::new(
            Vector2::new(5.3, 8.2),
            Vector2::new(8.7, 15.6)
        )).floor()
    )
}

#[test]
fn test_vector2_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2::new(5.3, 8.2)).floor()
    )
}

#[test]
fn test_vector2int16_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2int16::new(5, 8)).floor()
    )
}

#[test]
fn test_vector3_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3::new(5.3, 8.2, 4.7)).floor()
    )
}

#[test]
fn test_vector3int16_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3int16::new(5, 8, 4)).floor()
    )
}

#[test]
fn test_cframe_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(CFrame::new(Vector3::new(5.2, 10.8, 15.2), Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2)
        ))).floor()
    )
}

#[test]
fn test_matrix3_floor() {
    insta::assert_yaml_snapshot!(
        Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2)
        ).floor()
    )
}

#[test]
fn test_color3_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3::new(5.2, 8.4, 3.7)).floor()
    )
}

#[test]
fn test_color3uint8_floor() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3uint8::new(12, 16, 2)).floor()
    )
}

// ceil
#[test]
fn test_f32_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(5.3f32).ceil()
    )
}

#[test]
fn test_udim_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim::new(10.5, 15)).ceil()
    )
}

#[test]
fn test_udim2_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim2::new(UDim::new(10.5, 15), UDim::new(10.5, 15))).ceil()
    )
}

#[test]
fn test_rect_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Rect::new(
            Vector2::new(5.3, 8.2),
            Vector2::new(8.7, 15.6)
        )).ceil()
    )
}

#[test]
fn test_vector2_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2::new(5.3, 8.2)).ceil()
    )
}

#[test]
fn test_vector2int16_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2int16::new(5, 8)).ceil()
    )
}

#[test]
fn test_vector3_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3::new(5.3, 8.2, 4.7)).ceil()
    )
}

#[test]
fn test_vector3int16_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3int16::new(5, 8, 4)).ceil()
    )
}

#[test]
fn test_cframe_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(CFrame::new(Vector3::new(5.2, 10.8, 15.2), Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2)
        ))).ceil()
    )
}

#[test]
fn test_matrix3_ceil() {
    insta::assert_yaml_snapshot!(
        Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2)
        ).ceil()
    )
}

#[test]
fn test_color3_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3::new(5.2, 8.4, 3.7)).ceil()
    )
}

#[test]
fn test_color3uint8_ceil() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3uint8::new(12, 16, 2)).ceil()
    )
}

// round
#[test]
fn test_f32_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(5.3f32).round()
    )
}

#[test]
fn test_udim_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim::new(10.5, 15)).round()
    )
}

#[test]
fn test_udim2_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim2::new(UDim::new(10.5, 15), UDim::new(10.5, 15))).round()
    )
}

#[test]
fn test_rect_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Rect::new(
            Vector2::new(5.3, 8.2),
            Vector2::new(8.7, 15.6)
        )).round()
    )
}

#[test]
fn test_vector2_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2::new(5.3, 8.2)).round()
    )
}

#[test]
fn test_vector2int16_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2int16::new(5, 8)).round()
    )
}

#[test]
fn test_vector3_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3::new(5.3, 8.2, 4.7)).round()
    )
}

#[test]
fn test_vector3int16_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3int16::new(5, 8, 4)).round()
    )
}

#[test]
fn test_cframe_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(CFrame::new(Vector3::new(5.2, 10.8, 15.2), Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2)
        ))).round()
    )
}

#[test]
fn test_matrix3_round() {
    insta::assert_yaml_snapshot!(
        Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, 18.2)
        ).round()
    )
}

#[test]
fn test_color3_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3::new(5.2, 8.4, 3.7)).round()
    )
}

#[test]
fn test_color3uint8_round() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3uint8::new(12, 16, 2)).round()
    )
}

// abs
#[test]
fn test_f32_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(-5.3f32).abs()
    )
}

#[test]
fn test_udim_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim::new(10.5, -15)).abs()
    )
}

#[test]
fn test_udim2_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(UDim2::new(UDim::new(10.5, -15), UDim::new(-10.5, 15))).abs()
    )
}

#[test]
fn test_rect_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Rect::new(
            Vector2::new(-5.3, 8.2),
            Vector2::new(-8.7, -15.6)
        )).abs()
    )
}

#[test]
fn test_vector2_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2::new(5.3, -8.2)).abs()
    )
}

#[test]
fn test_vector2int16_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector2int16::new(5, -8)).abs()
    )
}

#[test]
fn test_vector3_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3::new(-5.3, 8.2, -4.7)).abs()
    )
}

#[test]
fn test_vector3int16_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Vector3int16::new(-5, -8, 4)).abs()
    )
}

#[test]
fn test_cframe_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(CFrame::new(Vector3::new(5.2, 10.8, 15.2), Matrix3::new(
            Vector3::new(-6.6, 12.7, 18.2),
            Vector3::new(6.6, 12.7, -18.2),
            Vector3::new(-6.6, 12.7, 18.2)
        ))).abs()
    )
}

#[test]
fn test_matrix3_abs() {
    insta::assert_yaml_snapshot!(
        Matrix3::new(
            Vector3::new(6.6, 12.7, 18.2),
            Vector3::new(6.6, -12.7, 18.2),
            Vector3::new(-6.6, 12.7, 18.2)
        ).abs()
    )
}

#[test]
fn test_color3_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3::new(5.2, 8.4, -3.7)).abs()
    )
}

#[test]
fn test_color3uint8_abs() {
    insta::assert_yaml_snapshot!(
        Variant::from(Color3uint8::new(12, 16, 2)).abs()
    )
}