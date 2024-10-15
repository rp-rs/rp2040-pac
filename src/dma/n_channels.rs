#[doc = "Register `N_CHANNELS` reader"]
pub type R = crate::R<N_CHANNELS_SPEC>;
#[doc = "Field `N_CHANNELS` reader - "]
pub type N_CHANNELS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn n_channels(&self) -> N_CHANNELS_R {
        N_CHANNELS_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area.  

You can [`read`](crate::Reg::read) this register and get [`n_channels::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct N_CHANNELS_SPEC;
impl crate::RegisterSpec for N_CHANNELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_channels::R`](R) reader structure"]
impl crate::Readable for N_CHANNELS_SPEC {}
#[doc = "`reset()` method sets N_CHANNELS to value 0"]
impl crate::Resettable for N_CHANNELS_SPEC {
    const RESET_VALUE: u32 = 0;
}
