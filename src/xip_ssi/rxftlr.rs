#[doc = "Register `RXFTLR` reader"]
pub struct R(crate::R<RXFTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFTLR` writer"]
pub struct W(crate::W<RXFTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFTLR_SPEC>;
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
impl From<crate::W<RXFTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFT` reader - Receive FIFO threshold"]
pub type RFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFT` writer - Receive FIFO threshold"]
pub type RFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXFTLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold"]
    #[inline(always)]
    pub fn rft(&self) -> RFT_R {
        RFT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rft(&mut self) -> RFT_W<0> {
        RFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO threshold level  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rxftlr](index.html) module"]
pub struct RXFTLR_SPEC;
impl crate::RegisterSpec for RXFTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftlr::R](R) reader structure"]
impl crate::Readable for RXFTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxftlr::W](W) writer structure"]
impl crate::Writable for RXFTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFTLR to value 0"]
impl crate::Resettable for RXFTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
