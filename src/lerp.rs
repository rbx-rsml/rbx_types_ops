use num_traits::Num;
use rbx_types::{CFrame, Color3, Color3uint8, Matrix3, Rect, UDim, UDim2, Variant, Vector2, Vector2int16, Vector3, Vector3int16};

macro_rules! implement_from_to_method_for_datatypes {
    ($trait_name:ident, $method_name:ident) => {
        impl $trait_name for f32 {
            fn $method_name(self, to: &f32, time: f32) -> Self {
                $method_name(self, *to, time)
            }
        }
        
        impl $trait_name for i32 {
            fn $method_name(self, to: &i32, time: f32) -> Self {
                $method_name(self as f32, *to as f32, time) as i32
            }
        }
        
        impl $trait_name for i16 {
            fn $method_name(self, to: &i16, time: f32) -> Self {
                $method_name(self as f32, *to as f32, time) as i16
            }
        }

        impl $trait_name for u8 {
            fn $method_name(self, to: &u8, time: f32) -> Self {
                $method_name(self as f32, *to as f32, time) as u8
            }
        }
        
        impl $trait_name for UDim {
            fn $method_name(self, to: &UDim, time: f32) -> Self {
                UDim::new(
                    self.scale.$method_name(&to.scale, time),
                    self.offset.$method_name(&to.offset, time)
                )
            }
        }
        
        impl $trait_name for UDim2 {
            fn $method_name(self, to: &UDim2, time: f32) -> Self {
                UDim2::new(
                    self.x.$method_name(&to.x, time),
                    self.y.$method_name(&to.y, time)
                )
            }
        }
        
        impl $trait_name for Rect {
            fn $method_name(self, to: &Rect, time: f32) -> Self {
                Rect::new(
                    self.min.$method_name(&to.min, time),
                    self.max.$method_name(&to.max, time),
                )
            }
        }
        
        impl $trait_name for Vector2 {
            fn $method_name(self, to: &Vector2, time: f32) -> Self {
                Vector2::new(
                    self.x.$method_name(&to.x, time),
                    self.y.$method_name(&to.y, time)
                )
            }
        }
        
        impl $trait_name for Vector2int16 {
            fn $method_name(self, to: &Vector2int16, time: f32) -> Self {
                Vector2int16::new(
                    self.x.$method_name(&to.x, time),
                    self.y.$method_name(&to.y, time)
                )
            }
        }
        
        impl $trait_name for Vector3 {
            fn $method_name(self, to: &Vector3, time: f32) -> Self {
                Vector3::new(
                    self.x.$method_name(&to.x, time),
                    self.y.$method_name(&to.y, time),
                    self.z.$method_name(&to.z, time)
                )
            }
        }
        
        impl $trait_name for Vector3int16 {
            fn $method_name(self, to: &Vector3int16, time: f32) -> Self {
                Vector3int16::new(
                    self.x.$method_name(&to.x, time),
                    self.y.$method_name(&to.y, time),
                    self.z.$method_name(&to.z, time)
                )
            }
        }
        
        impl $trait_name for Matrix3 {
            fn $method_name(self, to: &Matrix3, time: f32) -> Self {
                Matrix3::new(
                    self.x.$method_name(&to.x, time),
                    self.y.$method_name(&to.y, time),
                    self.z.$method_name(&to.z, time)
                )
            }
        }
        
        impl $trait_name for CFrame {
            fn $method_name(self, to: &CFrame, time: f32) -> Self {
                CFrame::new(
                    self.position.$method_name(&to.position, time),
                    self.orientation.$method_name(&to.orientation, time)
                )
            }
        }
        
        impl $trait_name for Color3 {
            fn $method_name(self, to: &Color3, time: f32) -> Self {
                Color3::new(
                    self.r.$method_name(&to.r, time),
                    self.g.$method_name(&to.g, time),
                    self.b.$method_name(&to.b, time),
                )
            }
        }

        impl $trait_name for Color3uint8 {
            fn $method_name(self, to: &Color3uint8, time: f32) -> Self {
                Color3uint8::new(
                    self.r.$method_name(&to.r, time),
                    self.g.$method_name(&to.g, time),
                    self.b.$method_name(&to.b, time),
                )
            }
        }

        impl $trait_name for Variant {
            fn $method_name(self, to: &Variant, time: f32) -> Self {
                match self {
                    Variant::Float32(from) => {
                        match to {
                            Variant::Float32(to) => Variant::Float32(from.$method_name(to, time)),
                            _ => Variant::Float32(from)
                        }
                    },
                    Variant::UDim(from) => {
                        match to {
                            Variant::UDim(to) => Variant::UDim(from.$method_name(to, time)),
                            _ => Variant::UDim(from)
                        }
                    },
                    Variant::UDim2(from) => {
                        match to {
                            Variant::UDim2(to) => Variant::UDim2(from.$method_name(to, time)),
                            _ => Variant::UDim2(from)
                        }
                    },
                    Variant::Vector3(from) => {
                        match to {
                            Variant::Vector3(to) => Variant::Vector3(from.$method_name(to, time)),
                            _ => Variant::Vector3(from)
                        }
                    },
                    Variant::Vector3int16(from) => {
                        match to {
                            Variant::Vector3int16(to) => Variant::Vector3int16(from.$method_name(to, time)),
                            _ => Variant::Vector3int16(from)
                        }
                    },
                    Variant::CFrame(from) => {
                        match to {
                            Variant::CFrame(to) => Variant::CFrame(from.$method_name(to, time)),
                            _ => Variant::CFrame(from)
                        }
                    },
                    Variant::Vector2(from) => {
                        match to {
                            Variant::Vector2(to) => Variant::Vector2(from.$method_name(to, time)),
                            _ => Variant::Vector2(from)
                        }
                    },
                    Variant::Vector2int16(from) => {
                        match to {
                            Variant::Vector2int16(to) => Variant::Vector2int16(from.$method_name(to, time)),
                            _ => Variant::Vector2int16(from)
                        }
                    },
                    Variant::Rect(from) => {
                        match to {
                            Variant::Rect(to) => Variant::Rect(from.$method_name(to, time)),
                            _ => Variant::Rect(from)
                        }
                    },
                    Variant::Color3(from) => {
                        match to {
                            Variant::Color3(to) => Variant::Color3(from.$method_name(to, time)),
                            _ => Variant::Color3(from)
                        }
                    },
                    Variant::Color3uint8(from) => {
                        match to {
                            Variant::Color3uint8(to) => Variant::Color3uint8(from.$method_name(to, time)),
                            _ => Variant::Color3uint8(from)
                        }
                    },
                    _ => self
                }
            }
        }
    };
}

fn lerp<N: Num + Copy>(from: N, to: N, time: N) -> N {
    from + (to - from) * time
}
pub trait Lerp {
    fn lerp(self, to: &Self, time: f32) -> Self;
}
implement_from_to_method_for_datatypes!(Lerp, lerp);