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
pub struct RSD_R(crate::FieldReader<u8, u8>);
impl RSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSD` writer - RXD sample delay (in SCLK cycles)"]
pub struct RSD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn rsd(&mut self) -> RSD_W {
        RSD_W { w: self }
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
}
#[doc = "`reset()` method sets RX_SAMPLE_DLY to value 0"]
impl crate::Resettable for RX_SAMPLE_DLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
