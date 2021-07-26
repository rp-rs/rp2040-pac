#[doc = "Register `ALARM3` reader"]
pub struct R(crate::R<ALARM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM3` writer"]
pub struct W(crate::W<ALARM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM3_SPEC>;
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
impl From<crate::W<ALARM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM3_SPEC>) -> Self {
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
#[doc = "Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [alarm3](index.html) module"]
pub struct ALARM3_SPEC;
impl crate::RegisterSpec for ALARM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm3::R](R) reader structure"]
impl crate::Readable for ALARM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm3::W](W) writer structure"]
impl crate::Writable for ALARM3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM3 to value 0"]
impl crate::Resettable for ALARM3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
