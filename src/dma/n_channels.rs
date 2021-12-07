#[doc = "Register `N_CHANNELS` reader"]
pub struct R(crate::R<N_CHANNELS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<N_CHANNELS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<N_CHANNELS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<N_CHANNELS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `N_CHANNELS` reader - "]
pub struct N_CHANNELS_R(crate::FieldReader<u8, u8>);
impl N_CHANNELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        N_CHANNELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_CHANNELS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn n_channels(&self) -> N_CHANNELS_R {
        N_CHANNELS_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [n_channels](index.html) module"]
pub struct N_CHANNELS_SPEC;
impl crate::RegisterSpec for N_CHANNELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [n_channels::R](R) reader structure"]
impl crate::Readable for N_CHANNELS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets N_CHANNELS to value 0"]
impl crate::Resettable for N_CHANNELS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
