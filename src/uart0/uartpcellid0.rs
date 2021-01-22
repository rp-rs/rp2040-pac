#[doc = "Reader of register UARTPCELLID0"]
pub type R = crate::R<u32, super::UARTPCELLID0>;
#[doc = "Reader of field `UARTPCELLID0`"]
pub type UARTPCELLID0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn uartpcellid0(&self) -> UARTPCELLID0_R {
        UARTPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
