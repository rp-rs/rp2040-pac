#[doc = "Register `STREAM_ADDR` reader"]
pub struct R(crate::R<STREAM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STREAM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STREAM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STREAM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STREAM_ADDR` writer"]
pub struct W(crate::W<STREAM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STREAM_ADDR_SPEC>;
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
impl From<crate::W<STREAM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STREAM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STREAM_ADDR` reader - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
pub struct STREAM_ADDR_R(crate::FieldReader<u32, u32>);
impl STREAM_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STREAM_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STREAM_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STREAM_ADDR` writer - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
pub struct STREAM_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> STREAM_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub fn stream_addr(&self) -> STREAM_ADDR_R {
        STREAM_ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub fn stream_addr(&mut self) -> STREAM_ADDR_W {
        STREAM_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO stream address  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [stream_addr](index.html) module"]
pub struct STREAM_ADDR_SPEC;
impl crate::RegisterSpec for STREAM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stream_addr::R](R) reader structure"]
impl crate::Readable for STREAM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stream_addr::W](W) writer structure"]
impl crate::Writable for STREAM_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STREAM_ADDR to value 0"]
impl crate::Resettable for STREAM_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
