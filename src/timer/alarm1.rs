#[doc = "Register `ALARM1` reader"]
pub struct R(crate::R<ALARM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM1` writer"]
pub struct W(crate::W<ALARM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM1_SPEC>;
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
impl From<crate::W<ALARM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM1_SPEC>) -> Self {
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
#[doc = "Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [alarm1](index.html) module"]
pub struct ALARM1_SPEC;
impl crate::RegisterSpec for ALARM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm1::R](R) reader structure"]
impl crate::Readable for ALARM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm1::W](W) writer structure"]
impl crate::Writable for ALARM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM1 to value 0"]
impl crate::Resettable for ALARM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
