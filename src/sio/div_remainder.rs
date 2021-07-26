#[doc = "Register `DIV_REMAINDER` reader"]
pub struct R(crate::R<DIV_REMAINDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_REMAINDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_REMAINDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_REMAINDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_REMAINDER` writer"]
pub struct W(crate::W<DIV_REMAINDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_REMAINDER_SPEC>;
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
impl From<crate::W<DIV_REMAINDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_REMAINDER_SPEC>) -> Self {
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
#[doc = "Divider result remainder  
 The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.  
 For signed calculations, REMAINDER is negative only when DIVIDEND is negative.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div_remainder](index.html) module"]
pub struct DIV_REMAINDER_SPEC;
impl crate::RegisterSpec for DIV_REMAINDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_remainder::R](R) reader structure"]
impl crate::Readable for DIV_REMAINDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_remainder::W](W) writer structure"]
impl crate::Writable for DIV_REMAINDER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_REMAINDER to value 0"]
impl crate::Resettable for DIV_REMAINDER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
