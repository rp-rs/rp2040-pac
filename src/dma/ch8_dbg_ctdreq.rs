#[doc = "Register `CH8_DBG_CTDREQ` reader"]
pub struct R(crate::R<CH8_DBG_CTDREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH8_DBG_CTDREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH8_DBG_CTDREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH8_DBG_CTDREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH8_DBG_CTDREQ` writer"]
pub struct W(crate::W<CH8_DBG_CTDREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH8_DBG_CTDREQ_SPEC>;
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
impl From<crate::W<CH8_DBG_CTDREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH8_DBG_CTDREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH8_DBG_CTDREQ` reader - "]
pub struct CH8_DBG_CTDREQ_R(crate::FieldReader<u8, u8>);
impl CH8_DBG_CTDREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH8_DBG_CTDREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH8_DBG_CTDREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH8_DBG_CTDREQ` writer - "]
pub struct CH8_DBG_CTDREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_DBG_CTDREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch8_dbg_ctdreq(&self) -> CH8_DBG_CTDREQ_R {
        CH8_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch8_dbg_ctdreq(&mut self) -> CH8_DBG_CTDREQ_W {
        CH8_DBG_CTDREQ_W { w: self }
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

For information about available fields see [ch8_dbg_ctdreq](index.html) module"]
pub struct CH8_DBG_CTDREQ_SPEC;
impl crate::RegisterSpec for CH8_DBG_CTDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch8_dbg_ctdreq::R](R) reader structure"]
impl crate::Readable for CH8_DBG_CTDREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch8_dbg_ctdreq::W](W) writer structure"]
impl crate::Writable for CH8_DBG_CTDREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH8_DBG_CTDREQ to value 0"]
impl crate::Resettable for CH8_DBG_CTDREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
