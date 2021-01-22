#[doc = "Reader of register UARTPERIPHID1"]
pub type R = crate::R<u32, super::UARTPERIPHID1>;
#[doc = "Reader of field `DESIGNER0`"]
pub type DESIGNER0_R = crate::R<u8, u8>;
#[doc = "Reader of field `PARTNUMBER1`"]
pub type PARTNUMBER1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> DESIGNER0_R {
        DESIGNER0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - These bits read back as 0x0"]
    #[inline(always)]
    pub fn partnumber1(&self) -> PARTNUMBER1_R {
        PARTNUMBER1_R::new((self.bits & 0x0f) as u8)
    }
}
