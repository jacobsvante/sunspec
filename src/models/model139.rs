/// LVRTX
///
/// LVRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model139 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub crvtype: u16,
}

#[allow(missing_docs)]

impl Model139 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRVTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model139 {
    const ID: u16 = 139;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            crvtype: Self::CRVTYPE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}