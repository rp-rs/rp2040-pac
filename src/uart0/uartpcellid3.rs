#[doc = "Reader of register UARTPCELLID3"]
pub type R = crate::R<u32, super::UARTPCELLID3>;
#[doc = "Reader of field `UARTPCELLID3`"]
pub type UARTPCELLID3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn uartpcellid3(&self) -> UARTPCELLID3_R {
        UARTPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
