#[doc = "Register `N_CHANNELS` reader"]
pub type R = crate::R<N_CHANNELS_SPEC>;
#[doc = "Register `N_CHANNELS` writer"]
pub type W = crate::W<N_CHANNELS_SPEC>;
#[doc = "Field `N_CHANNELS` reader - "]
pub type N_CHANNELS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn n_channels(&self) -> N_CHANNELS_R {
        N_CHANNELS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area.  

You can [`read`](crate::generic::Reg::read) this register and get [`n_channels::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_channels::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct N_CHANNELS_SPEC;
impl crate::RegisterSpec for N_CHANNELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_channels::R`](R) reader structure"]
impl crate::Readable for N_CHANNELS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`n_channels::W`](W) writer structure"]
impl crate::Writable for N_CHANNELS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets N_CHANNELS to value 0"]
impl crate::Resettable for N_CHANNELS_SPEC {
    const RESET_VALUE: u32 = 0;
}
