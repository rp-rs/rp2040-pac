#[doc = "Register `DORMANT` reader"]
pub struct R(crate::R<DORMANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DORMANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DORMANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DORMANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DORMANT` writer"]
pub struct W(crate::W<DORMANT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DORMANT_SPEC>;
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
impl From<crate::W<DORMANT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DORMANT_SPEC>) -> Self {
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
#[doc = "Ring Oscillator pause control  
 This is used to save power by pausing the ROSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 Warning: setup the irq before selecting dormant mode  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dormant](index.html) module"]
pub struct DORMANT_SPEC;
impl crate::RegisterSpec for DORMANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dormant::R](R) reader structure"]
impl crate::Readable for DORMANT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dormant::W](W) writer structure"]
impl crate::Writable for DORMANT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DORMANT to value 0"]
impl crate::Resettable for DORMANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
