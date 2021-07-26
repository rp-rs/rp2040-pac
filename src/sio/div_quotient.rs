#[doc = "Register `DIV_QUOTIENT` reader"]
pub struct R(crate::R<DIV_QUOTIENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_QUOTIENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_QUOTIENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_QUOTIENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_QUOTIENT` writer"]
pub struct W(crate::W<DIV_QUOTIENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_QUOTIENT_SPEC>;
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
impl From<crate::W<DIV_QUOTIENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_QUOTIENT_SPEC>) -> Self {
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
#[doc = "Divider result quotient  
 The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.  
 For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  
 Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order  
 REMAINDER, QUOTIENT if CSR_DIRTY is used.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div_quotient](index.html) module"]
pub struct DIV_QUOTIENT_SPEC;
impl crate::RegisterSpec for DIV_QUOTIENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_quotient::R](R) reader structure"]
impl crate::Readable for DIV_QUOTIENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_quotient::W](W) writer structure"]
impl crate::Writable for DIV_QUOTIENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_QUOTIENT to value 0"]
impl crate::Resettable for DIV_QUOTIENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
