#[doc = "Register `DIV_SDIVIDEND` reader"]
pub struct R(crate::R<DIV_SDIVIDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SDIVIDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SDIVIDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SDIVIDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_SDIVIDEND` writer"]
pub struct W(crate::W<DIV_SDIVIDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SDIVIDEND_SPEC>;
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
impl From<crate::W<DIV_SDIVIDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SDIVIDEND_SPEC>) -> Self {
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
#[doc = "Divider signed dividend  
 The same as UDIVIDEND, but starts a signed calculation, rather than unsigned.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div_sdividend](index.html) module"]
pub struct DIV_SDIVIDEND_SPEC;
impl crate::RegisterSpec for DIV_SDIVIDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_sdividend::R](R) reader structure"]
impl crate::Readable for DIV_SDIVIDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_sdividend::W](W) writer structure"]
impl crate::Writable for DIV_SDIVIDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_SDIVIDEND to value 0"]
impl crate::Resettable for DIV_SDIVIDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
