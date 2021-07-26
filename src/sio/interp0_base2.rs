#[doc = "Register `INTERP0_BASE2` reader"]
pub struct R(crate::R<INTERP0_BASE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP0_BASE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP0_BASE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP0_BASE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP0_BASE2` writer"]
pub struct W(crate::W<INTERP0_BASE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP0_BASE2_SPEC>;
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
impl From<crate::W<INTERP0_BASE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERP0_BASE2_SPEC>) -> Self {
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
#[doc = "Read/write access to BASE2 register.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp0_base2](index.html) module"]
pub struct INTERP0_BASE2_SPEC;
impl crate::RegisterSpec for INTERP0_BASE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp0_base2::R](R) reader structure"]
impl crate::Readable for INTERP0_BASE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp0_base2::W](W) writer structure"]
impl crate::Writable for INTERP0_BASE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERP0_BASE2 to value 0"]
impl crate::Resettable for INTERP0_BASE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
