#[doc = "Register `CH1_DBG_CTDREQ` reader"]
pub struct R(crate::R<CH1_DBG_CTDREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_DBG_CTDREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_DBG_CTDREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_DBG_CTDREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_DBG_CTDREQ` writer"]
pub struct W(crate::W<CH1_DBG_CTDREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_DBG_CTDREQ_SPEC>;
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
impl From<crate::W<CH1_DBG_CTDREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_DBG_CTDREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1_DBG_CTDREQ` reader - "]
pub type CH1_DBG_CTDREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_DBG_CTDREQ` writer - "]
pub type CH1_DBG_CTDREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH1_DBG_CTDREQ_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch1_dbg_ctdreq(&self) -> CH1_DBG_CTDREQ_R {
        CH1_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dbg_ctdreq(&mut self) -> CH1_DBG_CTDREQ_W<0> {
        CH1_DBG_CTDREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch1_dbg_ctdreq](index.html) module"]
pub struct CH1_DBG_CTDREQ_SPEC;
impl crate::RegisterSpec for CH1_DBG_CTDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_dbg_ctdreq::R](R) reader structure"]
impl crate::Readable for CH1_DBG_CTDREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_dbg_ctdreq::W](W) writer structure"]
impl crate::Writable for CH1_DBG_CTDREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3f;
}
#[doc = "`reset()` method sets CH1_DBG_CTDREQ to value 0"]
impl crate::Resettable for CH1_DBG_CTDREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
