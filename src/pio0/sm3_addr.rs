#[doc = "Reader of register SM3_ADDR"]
pub type R = crate::R<u32, super::SM3_ADDR>;
#[doc = "Reader of field `SM3_ADDR`"]
pub type SM3_ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm3_addr(&self) -> SM3_ADDR_R {
        SM3_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
