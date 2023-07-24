#[doc = "Register `RX_SAMPLE_DLY` reader"]
pub struct R(crate::R<RX_SAMPLE_DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SAMPLE_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SAMPLE_DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SAMPLE_DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_SAMPLE_DLY` writer"]
pub struct W(crate::W<RX_SAMPLE_DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SAMPLE_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RX_SAMPLE_DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SAMPLE_DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSD` reader - RXD sample delay (in SCLK cycles)"]
pub type RSD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSD` writer - RXD sample delay (in SCLK cycles)"]
pub type RSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_SAMPLE_DLY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    pub fn rsd(&self) -> RSD_R {
        RSD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    #[must_use]
    pub fn rsd(&mut self) -> RSD_W<0> {
        RSD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX sample delay  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rx_sample_dly](index.html) module"]
pub struct RX_SAMPLE_DLY_SPEC;
impl crate::RegisterSpec for RX_SAMPLE_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_sample_dly::R](R) reader structure"]
impl crate::Readable for RX_SAMPLE_DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_sample_dly::W](W) writer structure"]
impl crate::Writable for RX_SAMPLE_DLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_SAMPLE_DLY to value 0"]
impl crate::Resettable for RX_SAMPLE_DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
