#[doc = "Reader of register UARTPCELLID2"]
pub type R = crate::R<u32, super::UARTPCELLID2>;
#[doc = "Reader of field `UARTPCELLID2`"]
pub type UARTPCELLID2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn uartpcellid2(&self) -> UARTPCELLID2_R {
        UARTPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
