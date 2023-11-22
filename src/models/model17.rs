//! Serial Interface

/// Serial Interface
///
/// Include this model for serial interface configuration support
#[derive(Debug)]
pub struct Model17 {
    /// Name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Rate
    ///
    /// Interface baud rate in bits per second
    pub rte: u32,
    /// Bits
    ///
    /// Number of data bits per character
    pub bits: u16,
    /// Parity
    ///
    /// Bitmask value.  Parity setting
    pub pty: Pty,
    /// Duplex
    ///
    /// Enumerated value.  Duplex mode
    pub dup: Option<Dup>,
    /// Flow Control
    ///
    /// Flow Control Method
    pub flw: Option<Flw>,
    /// Interface Type
    ///
    /// Enumerated value.  Interface type
    pub typ: Option<Typ>,
    /// Protocol
    ///
    /// Enumerated value. Serial protocol selection
    pub pcol: Option<Pcol>,
}

#[allow(missing_docs)]

impl Model17 {
    pub const NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(0, 4, true);
    pub const RTE: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, true);
    pub const BITS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const PTY: crate::PointDef<Self, Pty> = crate::PointDef::new(7, 1, true);
    pub const DUP: crate::PointDef<Self, Option<Dup>> = crate::PointDef::new(8, 1, true);
    pub const FLW: crate::PointDef<Self, Option<Flw>> = crate::PointDef::new(9, 1, true);
    pub const TYP: crate::PointDef<Self, Option<Typ>> = crate::PointDef::new(10, 1, false);
    pub const PCOL: crate::PointDef<Self, Option<Pcol>> = crate::PointDef::new(11, 1, false);
}

impl crate::Model for Model17 {
    const ID: u16 = 17;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            rte: Self::RTE.from_data(data)?,
            bits: Self::BITS.from_data(data)?,
            pty: Self::PTY.from_data(data)?,
            dup: Self::DUP.from_data(data)?,
            flw: Self::FLW.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
            pcol: Self::PCOL.from_data(data)?,
        })
    }
}

#[doc = "Parity\n\nBitmask value.  Parity setting"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Pty {
    #[doc = ""]
    None = 0,
    #[doc = ""]
    Odd = 1,
    #[doc = ""]
    Even = 2,
}
impl crate::Value for Pty {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Pty> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Pty::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}

#[doc = "Duplex\n\nEnumerated value.  Duplex mode"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Dup {
    #[doc = ""]
    Full = 0,
    #[doc = ""]
    Half = 1,
}
impl crate::Value for Dup {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Dup> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Dup::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}

#[doc = "Flow Control\n\nFlow Control Method"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Flw {
    #[doc = ""]
    None = 0,
    #[doc = ""]
    Hw = 1,
    #[doc = ""]
    Xonxoff = 2,
}
impl crate::Value for Flw {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Flw> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Flw::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}

#[doc = "Interface Type\n\nEnumerated value.  Interface type"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Typ {
    #[doc = ""]
    Unknown = 0,
    #[doc = ""]
    Rs232 = 1,
    #[doc = ""]
    Rs485 = 2,
}
impl crate::Value for Typ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Typ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Typ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}

#[doc = "Protocol\n\nEnumerated value. Serial protocol selection"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Pcol {
    #[doc = ""]
    Unknown = 0,
    #[doc = ""]
    Modbus = 1,
    #[doc = ""]
    Vendor = 2,
}
impl crate::Value for Pcol {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Pcol> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Pcol::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
