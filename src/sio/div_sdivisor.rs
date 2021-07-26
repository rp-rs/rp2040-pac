#[doc = "Register `DIV_SDIVISOR` reader"]
pub struct R(crate::R<DIV_SDIVISOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SDIVISOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SDIVISOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SDIVISOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_SDIVISOR` writer"]
pub struct W(crate::W<DIV_SDIVISOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SDIVISOR_SPEC>;
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
impl From<crate::W<DIV_SDIVISOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SDIVISOR_SPEC>) -> Self {
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
#[doc = "Divider signed divisor  
 The same as UDIVISOR, but starts a signed calculation, rather than unsigned.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div_sdivisor](index.html) module"]
pub struct DIV_SDIVISOR_SPEC;
impl crate::RegisterSpec for DIV_SDIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_sdivisor::R](R) reader structure"]
impl crate::Readable for DIV_SDIVISOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_sdivisor::W](W) writer structure"]
impl crate::Writable for DIV_SDIVISOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_SDIVISOR to value 0"]
impl crate::Resettable for DIV_SDIVISOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
