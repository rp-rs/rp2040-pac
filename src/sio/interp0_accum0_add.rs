#[doc = "Register `INTERP0_ACCUM0_ADD` reader"]
pub struct R(crate::R<INTERP0_ACCUM0_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP0_ACCUM0_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP0_ACCUM0_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP0_ACCUM0_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP0_ACCUM0_ADD` writer"]
pub struct W(crate::W<INTERP0_ACCUM0_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP0_ACCUM0_ADD_SPEC>;
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
impl From<crate::W<INTERP0_ACCUM0_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERP0_ACCUM0_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERP0_ACCUM0_ADD` reader - "]
pub struct INTERP0_ACCUM0_ADD_R(crate::FieldReader<u32, u32>);
impl INTERP0_ACCUM0_ADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTERP0_ACCUM0_ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERP0_ACCUM0_ADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERP0_ACCUM0_ADD` writer - "]
pub struct INTERP0_ACCUM0_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERP0_ACCUM0_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp0_accum0_add(&self) -> INTERP0_ACCUM0_ADD_R {
        INTERP0_ACCUM0_ADD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp0_accum0_add(&mut self) -> INTERP0_ACCUM0_ADD_W {
        INTERP0_ACCUM0_ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Values written here are atomically added to ACCUM0  
 Reading yields lane 0's raw shift and mask value (BASE0 not added).  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp0_accum0_add](index.html) module"]
pub struct INTERP0_ACCUM0_ADD_SPEC;
impl crate::RegisterSpec for INTERP0_ACCUM0_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp0_accum0_add::R](R) reader structure"]
impl crate::Readable for INTERP0_ACCUM0_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp0_accum0_add::W](W) writer structure"]
impl crate::Writable for INTERP0_ACCUM0_ADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERP0_ACCUM0_ADD to value 0"]
impl crate::Resettable for INTERP0_ACCUM0_ADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
