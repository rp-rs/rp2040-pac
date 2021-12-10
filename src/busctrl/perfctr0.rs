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
pub struct PERFCTR0_R(crate::FieldReader<u32, u32>);
impl PERFCTR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERFCTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERFCTR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERFCTR0` writer - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
pub struct PERFCTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFCTR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn perfctr0(&self) -> PERFCTR0_R {
        PERFCTR0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn perfctr0(&mut self) -> PERFCTR0_W {
        PERFCTR0_W { w: self }
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
}
#[doc = "`reset()` method sets PERFCTR0 to value 0"]
impl crate::Resettable for PERFCTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
