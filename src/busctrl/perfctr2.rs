#[doc = "Register `PERFCTR2` reader"]
pub struct R(crate::R<PERFCTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFCTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFCTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFCTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFCTR2` writer"]
pub struct W(crate::W<PERFCTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFCTR2_SPEC>;
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
impl From<crate::W<PERFCTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFCTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERFCTR2` reader - Busfabric saturating performance counter 2  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL2"]
pub type PERFCTR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERFCTR2` writer - Busfabric saturating performance counter 2  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL2"]
pub type PERFCTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERFCTR2_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 2  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL2"]
    #[inline(always)]
    pub fn perfctr2(&self) -> PERFCTR2_R {
        PERFCTR2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 2  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr2(&mut self) -> PERFCTR2_W<0> {
        PERFCTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus fabric performance counter 2  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [perfctr2](index.html) module"]
pub struct PERFCTR2_SPEC;
impl crate::RegisterSpec for PERFCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfctr2::R](R) reader structure"]
impl crate::Readable for PERFCTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perfctr2::W](W) writer structure"]
impl crate::Writable for PERFCTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PERFCTR2 to value 0"]
impl crate::Resettable for PERFCTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
