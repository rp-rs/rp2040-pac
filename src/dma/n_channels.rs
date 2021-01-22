#[doc = "Reader of register N_CHANNELS"]
pub type R = crate::R<u32, super::N_CHANNELS>;
#[doc = "Reader of field `N_CHANNELS`"]
pub type N_CHANNELS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn n_channels(&self) -> N_CHANNELS_R {
        N_CHANNELS_R::new((self.bits & 0x1f) as u8)
    }
}
