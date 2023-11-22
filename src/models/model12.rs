//! IPv4

/// IPv4
///
/// Include to support an IPv4 protocol stack on this interface
#[derive(Debug)]
pub struct Model12 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// Config Status
    ///
    /// Enumerated value.  Configuration status
    pub cfg_st: CfgSt,
    /// Change Status
    ///
    /// Bitmask value.  A configuration change is pending
    pub chg_st: u16,
    /// Config Capability
    ///
    /// Bitmask value. Identify capable sources of configuration
    pub cap: u16,
    /// IPv4 Config
    ///
    /// Enumerated value.  Configuration method used.
    pub cfg: Cfg,
    /// Control
    ///
    /// Configure use of services
    pub ctl: Ctl,
    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    pub addr: String,
    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    pub msk: String,
    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    pub gw: Option<String>,
    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    pub dns1: Option<String>,
    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    pub dns2: Option<String>,
    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    pub ntp1: Option<String>,
    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    pub ntp2: Option<String>,
    /// Domain
    ///
    /// Domain name (24 chars max)
    pub dom_nam: Option<String>,
    /// Host Name
    ///
    /// Host name (24 chars max)
    pub host_nam: Option<String>,
}

#[allow(missing_docs)]

impl Model12 {
    pub const NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(0, 4, true);
    pub const CFG_ST: crate::PointDef<Self, CfgSt> = crate::PointDef::new(4, 1, false);
    pub const CHG_ST: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const CAP: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const CFG: crate::PointDef<Self, Cfg> = crate::PointDef::new(7, 1, true);
    pub const CTL: crate::PointDef<Self, Ctl> = crate::PointDef::new(8, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(9, 8, true);
    pub const MSK: crate::PointDef<Self, String> = crate::PointDef::new(17, 8, true);
    pub const GW: crate::PointDef<Self, Option<String>> = crate::PointDef::new(25, 8, true);
    pub const DNS1: crate::PointDef<Self, Option<String>> = crate::PointDef::new(33, 8, true);
    pub const DNS2: crate::PointDef<Self, Option<String>> = crate::PointDef::new(41, 8, true);
    pub const NTP1: crate::PointDef<Self, Option<String>> = crate::PointDef::new(49, 12, true);
    pub const NTP2: crate::PointDef<Self, Option<String>> = crate::PointDef::new(61, 12, true);
    pub const DOM_NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(73, 12, true);
    pub const HOST_NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(85, 12, true);
}

impl crate::Model for Model12 {
    const ID: u16 = 12;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfg_st: Self::CFG_ST.from_data(data)?,
            chg_st: Self::CHG_ST.from_data(data)?,
            cap: Self::CAP.from_data(data)?,
            cfg: Self::CFG.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            addr: Self::ADDR.from_data(data)?,
            msk: Self::MSK.from_data(data)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            ntp1: Self::NTP1.from_data(data)?,
            ntp2: Self::NTP2.from_data(data)?,
            dom_nam: Self::DOM_NAM.from_data(data)?,
            host_nam: Self::HOST_NAM.from_data(data)?,
        })
    }
}

#[doc = "Config Status\n\nEnumerated value.  Configuration status"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum CfgSt {
    #[doc = ""]
    NotConfigured = 0,
    #[doc = ""]
    ValidSetting = 1,
    #[doc = ""]
    ValidHw = 2,
}
impl crate::Value for CfgSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CfgSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CfgSt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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

#[doc = "IPv4 Config\n\nEnumerated value.  Configuration method used."]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Cfg {
    #[doc = ""]
    Static = 0,
    #[doc = ""]
    Dhcp = 1,
    #[doc = ""]
    Bootp = 2,
    #[doc = ""]
    Zeroconf = 3,
}
impl crate::Value for Cfg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Cfg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Cfg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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

#[doc = "Control\n\nConfigure use of services"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Ctl {
    #[doc = ""]
    EnableDns = 0,
    #[doc = ""]
    EnableNtp = 1,
}
impl crate::Value for Ctl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Ctl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Ctl::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
