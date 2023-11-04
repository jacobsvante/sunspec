/// SunSpec Test Model 2
#[derive(Debug)]
pub struct Model63002;

#[allow(missing_docs)]

impl Model63002 {}

impl crate::Model for Model63002 {
    const ID: u16 = 63002;
    const LENGTH: u16 = 4;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}