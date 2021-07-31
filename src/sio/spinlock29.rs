#[doc = "Register `SPINLOCK29` reader"]
pub struct R(crate::R<SPINLOCK29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPINLOCK29` writer"]
pub struct W(crate::W<SPINLOCK29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPINLOCK29_SPEC>;
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
impl From<crate::W<SPINLOCK29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPINLOCK29_SPEC>) -> Self {
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
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [spinlock29](index.html) module"]
pub struct SPINLOCK29_SPEC;
impl crate::RegisterSpec for SPINLOCK29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock29::R](R) reader structure"]
impl crate::Readable for SPINLOCK29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spinlock29::W](W) writer structure"]
impl crate::Writable for SPINLOCK29_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPINLOCK29 to value 0"]
impl crate::Resettable for SPINLOCK29_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
