#[doc = "Register `FLUSH` reader"]
pub struct R(crate::R<FLUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLUSH` writer"]
pub struct W(crate::W<FLUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLUSH_SPEC>;
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
impl From<crate::W<FLUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUSH` reader - Write 1 to flush the cache. This clears the tag memory, but  
 the data memory retains its contents. (This means cache-as-SRAM  
 contents is not affected by flush or reset.)  
 Reading will hold the bus (stall the processor) until the flush  
 completes. Alternatively STAT can be polled until completion."]
pub struct FLUSH_R(crate::FieldReader<bool, bool>);
impl FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUSH` writer - Write 1 to flush the cache. This clears the tag memory, but  
 the data memory retains its contents. (This means cache-as-SRAM  
 contents is not affected by flush or reset.)  
 Reading will hold the bus (stall the processor) until the flush  
 completes. Alternatively STAT can be polled until completion."]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write 1 to flush the cache. This clears the tag memory, but  
 the data memory retains its contents. (This means cache-as-SRAM  
 contents is not affected by flush or reset.)  
 Reading will hold the bus (stall the processor) until the flush  
 completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to flush the cache. This clears the tag memory, but  
 the data memory retains its contents. (This means cache-as-SRAM  
 contents is not affected by flush or reset.)  
 Reading will hold the bus (stall the processor) until the flush  
 completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Flush control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [flush](index.html) module"]
pub struct FLUSH_SPEC;
impl crate::RegisterSpec for FLUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flush::R](R) reader structure"]
impl crate::Readable for FLUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flush::W](W) writer structure"]
impl crate::Writable for FLUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLUSH to value 0"]
impl crate::Resettable for FLUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
