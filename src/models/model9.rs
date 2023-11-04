/// Set Operator Security Certificate
///
/// Security model for PKI
#[derive(Debug)]
pub struct Model9 {
    /// Cert_UID
    ///
    /// User ID for this certificate
    pub certuid: u16,
    /// Cert_Role
    ///
    /// Role for this certificate
    pub certrole: u16,
    /// Format
    ///
    /// Format of this certificate
    pub fmt: u16,
    /// Type
    ///
    /// Type of this certificate
    pub typ: u16,
    /// Total Length
    ///
    /// Total Length of the Certificate
    ///
    /// Notes: In registers, zero padded.
    pub totln: u16,
    /// Fragment length
    ///
    /// Length of this fragment
    ///
    /// Notes: Maximum fragment length is 80 registers
    pub frgln: u16,
    /// Frag1
    ///
    /// First word of this fragment
    pub frg1: u16,
    #[allow(missing_docs)]
    pub frg2: u16,
    #[allow(missing_docs)]
    pub frg3: u16,
    #[allow(missing_docs)]
    pub frg4: u16,
    #[allow(missing_docs)]
    pub frg5: u16,
    #[allow(missing_docs)]
    pub frg6: u16,
    #[allow(missing_docs)]
    pub frg7: u16,
    #[allow(missing_docs)]
    pub frg8: u16,
    #[allow(missing_docs)]
    pub frg9: u16,
    #[allow(missing_docs)]
    pub frg10: u16,
    #[allow(missing_docs)]
    pub frg11: u16,
    #[allow(missing_docs)]
    pub frg12: u16,
    #[allow(missing_docs)]
    pub frg13: u16,
    #[allow(missing_docs)]
    pub frg14: u16,
    #[allow(missing_docs)]
    pub frg15: u16,
    #[allow(missing_docs)]
    pub frg16: u16,
    #[allow(missing_docs)]
    pub frg17: u16,
    #[allow(missing_docs)]
    pub frg18: u16,
    #[allow(missing_docs)]
    pub frg19: u16,
    #[allow(missing_docs)]
    pub frg20: u16,
    #[allow(missing_docs)]
    pub frg21: u16,
    #[allow(missing_docs)]
    pub frg22: u16,
    #[allow(missing_docs)]
    pub frg23: u16,
    #[allow(missing_docs)]
    pub frg24: u16,
    #[allow(missing_docs)]
    pub frg25: u16,
    #[allow(missing_docs)]
    pub frg26: u16,
    #[allow(missing_docs)]
    pub frg27: u16,
    #[allow(missing_docs)]
    pub frg28: u16,
    #[allow(missing_docs)]
    pub frg29: u16,
    #[allow(missing_docs)]
    pub frg30: u16,
    #[allow(missing_docs)]
    pub frg31: u16,
    #[allow(missing_docs)]
    pub frg32: u16,
    #[allow(missing_docs)]
    pub frg33: u16,
    #[allow(missing_docs)]
    pub frg34: u16,
    #[allow(missing_docs)]
    pub frg35: u16,
    #[allow(missing_docs)]
    pub frg36: u16,
    #[allow(missing_docs)]
    pub frg37: u16,
    #[allow(missing_docs)]
    pub frg38: u16,
    #[allow(missing_docs)]
    pub frg39: u16,
    #[allow(missing_docs)]
    pub frg40: u16,
    #[allow(missing_docs)]
    pub frg41: u16,
    #[allow(missing_docs)]
    pub frg42: u16,
    #[allow(missing_docs)]
    pub frg43: u16,
    #[allow(missing_docs)]
    pub frg44: u16,
    #[allow(missing_docs)]
    pub frg45: u16,
    #[allow(missing_docs)]
    pub frg46: u16,
    #[allow(missing_docs)]
    pub frg47: u16,
    #[allow(missing_docs)]
    pub frg48: u16,
    #[allow(missing_docs)]
    pub frg49: u16,
    #[allow(missing_docs)]
    pub frg50: u16,
    #[allow(missing_docs)]
    pub frg51: u16,
    #[allow(missing_docs)]
    pub frg52: u16,
    #[allow(missing_docs)]
    pub frg53: u16,
    #[allow(missing_docs)]
    pub frg54: u16,
    #[allow(missing_docs)]
    pub frg55: u16,
    #[allow(missing_docs)]
    pub frg56: u16,
    #[allow(missing_docs)]
    pub frg57: u16,
    #[allow(missing_docs)]
    pub frg58: u16,
    #[allow(missing_docs)]
    pub frg59: u16,
    #[allow(missing_docs)]
    pub frg60: u16,
    #[allow(missing_docs)]
    pub frg61: u16,
    #[allow(missing_docs)]
    pub frg62: u16,
    #[allow(missing_docs)]
    pub frg63: u16,
    #[allow(missing_docs)]
    pub frg64: u16,
    #[allow(missing_docs)]
    pub frg65: u16,
    #[allow(missing_docs)]
    pub frg66: u16,
    #[allow(missing_docs)]
    pub frg67: u16,
    #[allow(missing_docs)]
    pub frg68: u16,
    #[allow(missing_docs)]
    pub frg69: u16,
    #[allow(missing_docs)]
    pub frg70: u16,
    #[allow(missing_docs)]
    pub frg71: u16,
    #[allow(missing_docs)]
    pub frg72: u16,
    #[allow(missing_docs)]
    pub frg73: u16,
    #[allow(missing_docs)]
    pub frg74: u16,
    #[allow(missing_docs)]
    pub frg75: u16,
    #[allow(missing_docs)]
    pub frg78: u16,
    #[allow(missing_docs)]
    pub frg79: u16,
    /// Frag80
    ///
    /// Last word of this fragment
    pub frg80: u16,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    pub ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    pub ms: u16,
    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Notes: Shall be advanced for each request
    pub seq: u16,
    /// UID
    ///
    /// User ID for the request signature
    pub uid: u16,
    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Notes: Each controller is assigned a key index that maps to their access control role
    pub role: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Notes: For future proof
    pub alg: u16,
    /// N
    ///
    /// Number of registers to follow for the certificate
    pub n: u16,
}

#[allow(missing_docs)]

impl Model9 {
    pub const CERTUID: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const CERTROLE: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const FMT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const TOTLN: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const FRGLN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const FRG1: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const FRG2: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const FRG3: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const FRG4: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const FRG5: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const FRG6: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const FRG7: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const FRG8: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const FRG9: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, true);
    pub const FRG10: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const FRG11: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const FRG12: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const FRG13: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const FRG14: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const FRG15: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const FRG16: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, true);
    pub const FRG17: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const FRG18: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, true);
    pub const FRG19: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, true);
    pub const FRG20: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, true);
    pub const FRG21: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, true);
    pub const FRG22: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const FRG23: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, true);
    pub const FRG24: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, true);
    pub const FRG25: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, true);
    pub const FRG26: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, true);
    pub const FRG27: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, true);
    pub const FRG28: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, true);
    pub const FRG29: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const FRG30: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const FRG31: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, true);
    pub const FRG32: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, true);
    pub const FRG33: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, true);
    pub const FRG34: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, true);
    pub const FRG35: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, true);
    pub const FRG36: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, true);
    pub const FRG37: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, true);
    pub const FRG38: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, true);
    pub const FRG39: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, true);
    pub const FRG40: crate::PointDef<Self, u16> = crate::PointDef::new(45, 1, true);
    pub const FRG41: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, true);
    pub const FRG42: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, true);
    pub const FRG43: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const FRG44: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const FRG45: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
    pub const FRG46: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, true);
    pub const FRG47: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, true);
    pub const FRG48: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, true);
    pub const FRG49: crate::PointDef<Self, u16> = crate::PointDef::new(54, 1, true);
    pub const FRG50: crate::PointDef<Self, u16> = crate::PointDef::new(55, 1, true);
    pub const FRG51: crate::PointDef<Self, u16> = crate::PointDef::new(56, 1, true);
    pub const FRG52: crate::PointDef<Self, u16> = crate::PointDef::new(57, 1, true);
    pub const FRG53: crate::PointDef<Self, u16> = crate::PointDef::new(58, 1, true);
    pub const FRG54: crate::PointDef<Self, u16> = crate::PointDef::new(59, 1, true);
    pub const FRG55: crate::PointDef<Self, u16> = crate::PointDef::new(60, 1, true);
    pub const FRG56: crate::PointDef<Self, u16> = crate::PointDef::new(61, 1, true);
    pub const FRG57: crate::PointDef<Self, u16> = crate::PointDef::new(62, 1, true);
    pub const FRG58: crate::PointDef<Self, u16> = crate::PointDef::new(63, 1, true);
    pub const FRG59: crate::PointDef<Self, u16> = crate::PointDef::new(64, 1, true);
    pub const FRG60: crate::PointDef<Self, u16> = crate::PointDef::new(65, 1, true);
    pub const FRG61: crate::PointDef<Self, u16> = crate::PointDef::new(66, 1, true);
    pub const FRG62: crate::PointDef<Self, u16> = crate::PointDef::new(67, 1, true);
    pub const FRG63: crate::PointDef<Self, u16> = crate::PointDef::new(68, 1, true);
    pub const FRG64: crate::PointDef<Self, u16> = crate::PointDef::new(69, 1, true);
    pub const FRG65: crate::PointDef<Self, u16> = crate::PointDef::new(70, 1, true);
    pub const FRG66: crate::PointDef<Self, u16> = crate::PointDef::new(71, 1, true);
    pub const FRG67: crate::PointDef<Self, u16> = crate::PointDef::new(72, 1, true);
    pub const FRG68: crate::PointDef<Self, u16> = crate::PointDef::new(73, 1, true);
    pub const FRG69: crate::PointDef<Self, u16> = crate::PointDef::new(74, 1, true);
    pub const FRG70: crate::PointDef<Self, u16> = crate::PointDef::new(75, 1, true);
    pub const FRG71: crate::PointDef<Self, u16> = crate::PointDef::new(76, 1, true);
    pub const FRG72: crate::PointDef<Self, u16> = crate::PointDef::new(77, 1, true);
    pub const FRG73: crate::PointDef<Self, u16> = crate::PointDef::new(78, 1, true);
    pub const FRG74: crate::PointDef<Self, u16> = crate::PointDef::new(79, 1, true);
    pub const FRG75: crate::PointDef<Self, u16> = crate::PointDef::new(80, 1, true);
    pub const FRG78: crate::PointDef<Self, u16> = crate::PointDef::new(81, 1, true);
    pub const FRG79: crate::PointDef<Self, u16> = crate::PointDef::new(82, 1, true);
    pub const FRG80: crate::PointDef<Self, u16> = crate::PointDef::new(83, 1, true);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, true);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(86, 1, true);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(87, 1, true);
    pub const UID: crate::PointDef<Self, u16> = crate::PointDef::new(88, 1, true);
    pub const ROLE: crate::PointDef<Self, u16> = crate::PointDef::new(89, 1, true);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(90, 1, true);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(91, 1, true);
}

impl crate::Model for Model9 {
    const ID: u16 = 9;
    const LENGTH: u16 = 93;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            certuid: Self::CERTUID
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            certrole: Self::CERTROLE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            fmt: Self::FMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totln: Self::TOTLN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frgln: Self::FRGLN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg1: Self::FRG1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg2: Self::FRG2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg3: Self::FRG3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg4: Self::FRG4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg5: Self::FRG5
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg6: Self::FRG6
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg7: Self::FRG7
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg8: Self::FRG8
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg9: Self::FRG9
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg10: Self::FRG10
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg11: Self::FRG11
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg12: Self::FRG12
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg13: Self::FRG13
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg14: Self::FRG14
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg15: Self::FRG15
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg16: Self::FRG16
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg17: Self::FRG17
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg18: Self::FRG18
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg19: Self::FRG19
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg20: Self::FRG20
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg21: Self::FRG21
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg22: Self::FRG22
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg23: Self::FRG23
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg24: Self::FRG24
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg25: Self::FRG25
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg26: Self::FRG26
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg27: Self::FRG27
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg28: Self::FRG28
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg29: Self::FRG29
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg30: Self::FRG30
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg31: Self::FRG31
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg32: Self::FRG32
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg33: Self::FRG33
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg34: Self::FRG34
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg35: Self::FRG35
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg36: Self::FRG36
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg37: Self::FRG37
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg38: Self::FRG38
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg39: Self::FRG39
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg40: Self::FRG40
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg41: Self::FRG41
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg42: Self::FRG42
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg43: Self::FRG43
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg44: Self::FRG44
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg45: Self::FRG45
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg46: Self::FRG46
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg47: Self::FRG47
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg48: Self::FRG48
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg49: Self::FRG49
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg50: Self::FRG50
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg51: Self::FRG51
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg52: Self::FRG52
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg53: Self::FRG53
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg54: Self::FRG54
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg55: Self::FRG55
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg56: Self::FRG56
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg57: Self::FRG57
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg58: Self::FRG58
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg59: Self::FRG59
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg60: Self::FRG60
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg61: Self::FRG61
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg62: Self::FRG62
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg63: Self::FRG63
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg64: Self::FRG64
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg65: Self::FRG65
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg66: Self::FRG66
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg67: Self::FRG67
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg68: Self::FRG68
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg69: Self::FRG69
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg70: Self::FRG70
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg71: Self::FRG71
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg72: Self::FRG72
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg73: Self::FRG73
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg74: Self::FRG74
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg75: Self::FRG75
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg78: Self::FRG78
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg79: Self::FRG79
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg80: Self::FRG80
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ts: Self::TS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ms: Self::MS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            seq: Self::SEQ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            uid: Self::UID
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            role: Self::ROLE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alg: Self::ALG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
