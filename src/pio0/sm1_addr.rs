#[doc = "Reader of register SM1_ADDR"]
pub type R = crate::R<u32, super::SM1_ADDR>;
#[doc = "Reader of field `SM1_ADDR`"]
pub type SM1_ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm1_addr(&self) -> SM1_ADDR_R {
        SM1_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
