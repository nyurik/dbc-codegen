#![allow(dead_code)]

use std::fmt::Display;

use ValSize::{Size128, Size16, Size32, Size64, Size8};
use ValType::{Bool, SignedInt, UnsignedInt, F32, F64};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValSize {
    Size8,
    Size16,
    Size32,
    Size64,
    Size128,
}

impl ValSize {
    pub fn bits(self) -> u64 {
        match self {
            Size8 => 8,
            Size16 => 16,
            Size32 => 32,
            Size64 => 64,
            Size128 => 128,
        }
    }
}

/// Types available in DBC signals
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValType {
    Bool,
    UnsignedInt(ValSize),
    SignedInt(ValSize),
    F32,
    #[allow(dead_code)]
    F64,
}

impl ValType {
    pub fn bits(self) -> u64 {
        match self {
            Bool => 1,
            SignedInt(v) | UnsignedInt(v) => v.bits(),
            F32 => 32,
            F64 => 64,
        }
    }

    pub fn is_signed(self) -> bool {
        matches!(self, SignedInt(_))
    }

    #[allow(dead_code)]
    pub fn is_unsigned(self) -> bool {
        matches!(self, UnsignedInt(_))
    }

    #[allow(dead_code)]
    pub fn is_float(self) -> bool {
        matches!(self, F32 | F64)
    }

    pub fn unsigned_to_signed(self) -> Option<ValType> {
        if let UnsignedInt(size) = self {
            Some(SignedInt(size))
        } else {
            None
        }
    }

    pub fn signed_to_unsigned(self) -> Option<ValType> {
        if let SignedInt(size) = self {
            Some(UnsignedInt(size))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn format_value(self, value: f64) -> String {
        match self {
            Bool => format!("{}", (value - 1.0).abs() < f64::from(f32::EPSILON)),
            UnsignedInt(Size128) => format!("{}", value as u128),
            UnsignedInt(_) => format!("{}", value as u64),
            SignedInt(Size128) => format!("{}", value as i128),
            SignedInt(_) => format!("{}", value as i64),
            F32 | F64 => {
                if value.fract() == 0.0 {
                    format!("{value}.0")
                } else {
                    format!("{value}")
                }
            }
        }
    }
}

impl Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bool => write!(f, "bool"),
            UnsignedInt(v) => write!(f, "u{}", v.bits()),
            SignedInt(v) => write!(f, "i{}", v.bits()),
            F32 => write!(f, "f32"),
            F64 => write!(f, "f64"),
        }
    }
}
