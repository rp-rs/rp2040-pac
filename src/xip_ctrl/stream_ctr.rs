#[doc = "Register `STREAM_CTR` reader"]
pub struct R(crate::R<STREAM_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STREAM_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STREAM_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STREAM_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STREAM_CTR` writer"]
pub struct W(crate::W<STREAM_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STREAM_CTR_SPEC>;
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
impl From<crate::W<STREAM_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STREAM_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STREAM_CTR` reader - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
pub struct STREAM_CTR_R(crate::FieldReader<u32, u32>);
impl STREAM_CTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STREAM_CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STREAM_CTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STREAM_CTR` writer - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
pub struct STREAM_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> STREAM_CTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub fn stream_ctr(&self) -> STREAM_CTR_R {
        STREAM_CTR_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub fn stream_ctr(&mut self) -> STREAM_CTR_W {
        STREAM_CTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO stream control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [stream_ctr](index.html) module"]
pub struct STREAM_CTR_SPEC;
impl crate::RegisterSpec for STREAM_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stream_ctr::R](R) reader structure"]
impl crate::Readable for STREAM_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stream_ctr::W](W) writer structure"]
impl crate::Writable for STREAM_CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STREAM_CTR to value 0"]
impl crate::Resettable for STREAM_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
