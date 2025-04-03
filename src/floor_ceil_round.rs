use num_traits::Float;
use rbx_types::{CFrame, Color3, Matrix3, Rect, UDim, UDim2, Vector2, Vector3};

macro_rules! implement_method_for_datatypes {
    ($trait_name:ident, $method_name:ident) => {
        impl $trait_name for f32 {
            fn $method_name(&self) -> Self {
                Float::$method_name(*self)
            }
        }

        impl $trait_name for UDim {
            fn $method_name(&self) -> Self {
                UDim::new(
                    self.scale.$method_name(),
                    self.offset
                )
            }
        }
        
        impl $trait_name for UDim2 {
            fn $method_name(&self) -> Self {
                UDim2::new(
                    self.x.$method_name(),
                    self.y.$method_name()
                )
            }
        }
        
        impl $trait_name for Rect {
            fn $method_name(&self) -> Self {
                Rect::new(
                    self.min.$method_name(),
                    self.max.$method_name(),
                )
            }
        }
        
        impl $trait_name for Vector2 {
            fn $method_name(&self) -> Self {
                Vector2::new(
                    self.x.$method_name(),
                    self.y.$method_name()
                )
            }
        }
        
        impl $trait_name for Vector3 {
            fn $method_name(&self) -> Self {
                Vector3::new(
                    self.x.$method_name(),
                    self.y.$method_name(),
                    self.z.$method_name()
                )
            }
        }
        
        impl $trait_name for Matrix3 {
            fn $method_name(&self) -> Self {
                Matrix3::new(
                    self.x.$method_name(),
                    self.y.$method_name(),
                    self.z.$method_name()
                )
            }
        }
        
        impl $trait_name for CFrame {
            fn $method_name(&self) -> Self {
                CFrame::new(
                    self.position.$method_name(),
                    self.orientation.$method_name()
                )
            }
        }
        
        impl $trait_name for Color3 {
            fn $method_name(&self) -> Self {
                Color3::new(
                    self.r.$method_name(),
                    self.g.$method_name(),
                    self.b.$method_name(),
                )
            }
        }
    };
}

pub trait Ceil {
    fn ceil(&self) -> Self;
}
implement_method_for_datatypes!(Ceil, ceil);

pub trait Floor {
    fn floor(&self) -> Self;
}
implement_method_for_datatypes!(Floor, floor);

pub trait Round {
    fn round(&self) -> Self;
}
implement_method_for_datatypes!(Round, round);