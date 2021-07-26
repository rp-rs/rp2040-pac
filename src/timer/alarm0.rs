#[doc = "Register `ALARM0` reader"]
pub struct R(crate::R<ALARM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM0` writer"]
pub struct W(crate::W<ALARM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM0_SPEC>;
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
impl From<crate::W<ALARM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM0_SPEC>) -> Self {
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
#[doc = "Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [alarm0](index.html) module"]
pub struct ALARM0_SPEC;
impl crate::RegisterSpec for ALARM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm0::R](R) reader structure"]
impl crate::Readable for ALARM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm0::W](W) writer structure"]
impl crate::Writable for ALARM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM0 to value 0"]
impl crate::Resettable for ALARM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
