#[doc = "Reader of register SM0_ADDR"]
pub type R = crate::R<u32, super::SM0_ADDR>;
#[doc = "Reader of field `SM0_ADDR`"]
pub type SM0_ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm0_addr(&self) -> SM0_ADDR_R {
        SM0_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
