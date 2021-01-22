#[doc = "Reader of register UARTPERIPHID3"]
pub type R = crate::R<u32, super::UARTPERIPHID3>;
#[doc = "Reader of field `CONFIGURATION`"]
pub type CONFIGURATION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
