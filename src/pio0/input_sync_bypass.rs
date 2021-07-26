#[doc = "Register `INPUT_SYNC_BYPASS` reader"]
pub struct R(crate::R<INPUT_SYNC_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_SYNC_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_SYNC_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_SYNC_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUT_SYNC_BYPASS` writer"]
pub struct W(crate::W<INPUT_SYNC_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUT_SYNC_BYPASS_SPEC>;
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
impl From<crate::W<INPUT_SYNC_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUT_SYNC_BYPASS_SPEC>) -> Self {
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
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.  
 0 -> input is synchronized (default)  
 1 -> synchronizer is bypassed  
 If in doubt, leave this register as all zeroes.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [input_sync_bypass](index.html) module"]
pub struct INPUT_SYNC_BYPASS_SPEC;
impl crate::RegisterSpec for INPUT_SYNC_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input_sync_bypass::R](R) reader structure"]
impl crate::Readable for INPUT_SYNC_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [input_sync_bypass::W](W) writer structure"]
impl crate::Writable for INPUT_SYNC_BYPASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUT_SYNC_BYPASS to value 0"]
impl crate::Resettable for INPUT_SYNC_BYPASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
