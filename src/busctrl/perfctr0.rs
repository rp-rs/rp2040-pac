#[doc = "Register `PERFCTR0` reader"]
pub struct R(crate::R<PERFCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFCTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFCTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFCTR0` writer"]
pub struct W(crate::W<PERFCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFCTR0_SPEC>;
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
impl From<crate::W<PERFCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFCTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERFCTR0` reader - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
pub type PERFCTR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERFCTR0` writer - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
pub type PERFCTR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERFCTR0_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn perfctr0(&self) -> PERFCTR0_R {
        PERFCTR0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr0(&mut self) -> PERFCTR0_W<0> {
        PERFCTR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus fabric performance counter 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [perfctr0](index.html) module"]
pub struct PERFCTR0_SPEC;
impl crate::RegisterSpec for PERFCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfctr0::R](R) reader structure"]
impl crate::Readable for PERFCTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perfctr0::W](W) writer structure"]
impl crate::Writable for PERFCTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PERFCTR0 to value 0"]
impl crate::Resettable for PERFCTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
