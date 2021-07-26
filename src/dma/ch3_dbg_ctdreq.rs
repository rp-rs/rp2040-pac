#[doc = "Register `CH3_DBG_CTDREQ` reader"]
pub struct R(crate::R<CH3_DBG_CTDREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_DBG_CTDREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3_DBG_CTDREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3_DBG_CTDREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH3_DBG_CTDREQ` reader - "]
pub struct CH3_DBG_CTDREQ_R(crate::FieldReader<u8, u8>);
impl CH3_DBG_CTDREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH3_DBG_CTDREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_DBG_CTDREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch3_dbg_ctdreq(&self) -> CH3_DBG_CTDREQ_R {
        CH3_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch3_dbg_ctdreq](index.html) module"]
pub struct CH3_DBG_CTDREQ_SPEC;
impl crate::RegisterSpec for CH3_DBG_CTDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_dbg_ctdreq::R](R) reader structure"]
impl crate::Readable for CH3_DBG_CTDREQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH3_DBG_CTDREQ to value 0"]
impl crate::Resettable for CH3_DBG_CTDREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
