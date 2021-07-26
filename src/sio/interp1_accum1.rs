#[doc = "Register `INTERP1_ACCUM1` reader"]
pub struct R(crate::R<INTERP1_ACCUM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP1_ACCUM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP1_ACCUM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP1_ACCUM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP1_ACCUM1` writer"]
pub struct W(crate::W<INTERP1_ACCUM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP1_ACCUM1_SPEC>;
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
impl From<crate::W<INTERP1_ACCUM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERP1_ACCUM1_SPEC>) -> Self {
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
#[doc = "Read/write access to accumulator 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp1_accum1](index.html) module"]
pub struct INTERP1_ACCUM1_SPEC;
impl crate::RegisterSpec for INTERP1_ACCUM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp1_accum1::R](R) reader structure"]
impl crate::Readable for INTERP1_ACCUM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp1_accum1::W](W) writer structure"]
impl crate::Writable for INTERP1_ACCUM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERP1_ACCUM1 to value 0"]
impl crate::Resettable for INTERP1_ACCUM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
