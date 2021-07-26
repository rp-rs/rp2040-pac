#[doc = "Register `CTR_HIT` reader"]
pub struct R(crate::R<CTR_HIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_HIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_HIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_HIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR_HIT` writer"]
pub struct W(crate::W<CTR_HIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_HIT_SPEC>;
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
impl From<crate::W<CTR_HIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_HIT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Hit counter  
 A 32 bit saturating counter that increments upon each cache hit,  
 i.e. when an XIP access is serviced directly from cached data.  
 Write any value to clear.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctr_hit](index.html) module"]
pub struct CTR_HIT_SPEC;
impl crate::RegisterSpec for CTR_HIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr_hit::R](R) reader structure"]
impl crate::Readable for CTR_HIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr_hit::W](W) writer structure"]
impl crate::Writable for CTR_HIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTR_HIT to value 0"]
impl crate::Resettable for CTR_HIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
