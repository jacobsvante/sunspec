/// Eltek Inverter Extension
#[derive(Debug)]
pub struct Model64101 {
    #[allow(missing_docs)]
    pub eltek_country_code: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_feeding_phase: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_apd_method: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_apd_power_ref: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_method: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_q_ref: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_cosphi_ref: Option<i16>,
}

#[allow(missing_docs)]

impl Model64101 {
    pub const ELTEK_COUNTRY_CODE: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const ELTEK_FEEDING_PHASE: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ELTEK_APD_METHOD: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ELTEK_APD_POWER_REF: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const ELTEK_RPS_METHOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const ELTEK_RPS_Q_REF: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const ELTEK_RPS_COSPHI_REF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model64101 {
    const ID: u16 = 64101;
    const LENGTH: u16 = 7;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            eltek_country_code: Self::ELTEK_COUNTRY_CODE.from_data(data)?,
            eltek_feeding_phase: Self::ELTEK_FEEDING_PHASE.from_data(data)?,
            eltek_apd_method: Self::ELTEK_APD_METHOD.from_data(data)?,
            eltek_apd_power_ref: Self::ELTEK_APD_POWER_REF.from_data(data)?,
            eltek_rps_method: Self::ELTEK_RPS_METHOD.from_data(data)?,
            eltek_rps_q_ref: Self::ELTEK_RPS_Q_REF.from_data(data)?,
            eltek_rps_cosphi_ref: Self::ELTEK_RPS_COSPHI_REF.from_data(data)?,
        })
    }
}
