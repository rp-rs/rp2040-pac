#[doc = "Reader of register UARTPCELLID1"]
pub type R = crate::R<u32, super::UARTPCELLID1>;
#[doc = "Reader of field `UARTPCELLID1`"]
pub type UARTPCELLID1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn uartpcellid1(&self) -> UARTPCELLID1_R {
        UARTPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
