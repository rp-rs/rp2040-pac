#[doc = "Register `SYST_CSR` reader"]
pub struct R(crate::R<SYST_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CSR` writer"]
pub struct W(crate::W<SYST_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CSR_SPEC>;
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
impl From<crate::W<SYST_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYST_CSR_SPEC, bool, O>;
#[doc = "Field `TICKINT` reader - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
pub type TICKINT_R = crate::BitReader<bool>;
#[doc = "Field `TICKINT` writer - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
pub type TICKINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYST_CSR_SPEC, bool, O>;
#[doc = "Field `CLKSOURCE` reader - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
pub type CLKSOURCE_R = crate::BitReader<bool>;
#[doc = "Field `CLKSOURCE` writer - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
pub type CLKSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYST_CSR_SPEC, bool, O>;
#[doc = "Field `COUNTFLAG` reader - Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
pub type COUNTFLAG_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<1> {
        TICKINT_W::new(self)
    }
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<2> {
        CLKSOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [syst_csr](index.html) module"]
pub struct SYST_CSR_SPEC;
impl crate::RegisterSpec for SYST_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_csr::R](R) reader structure"]
impl crate::Readable for SYST_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](W) writer structure"]
impl crate::Writable for SYST_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_CSR to value 0"]
impl crate::Resettable for SYST_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
