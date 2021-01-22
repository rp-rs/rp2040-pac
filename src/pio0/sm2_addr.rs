#[doc = "Reader of register SM2_ADDR"]
pub type R = crate::R<u32, super::SM2_ADDR>;
#[doc = "Reader of field `SM2_ADDR`"]
pub type SM2_ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm2_addr(&self) -> SM2_ADDR_R {
        SM2_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
