#[doc = "Register `SYST_CSR` reader"]
pub type R = crate::R<SYST_CSR_SPEC>;
#[doc = "Register `SYST_CSR` writer"]
pub type W = crate::W<SYST_CSR_SPEC>;
#[doc = "Field `ENABLE` reader - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKINT` reader - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
pub type TICKINT_R = crate::BitReader;
#[doc = "Field `TICKINT` writer - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
pub type TICKINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSOURCE` reader - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
pub type CLKSOURCE_R = crate::BitReader;
#[doc = "Field `CLKSOURCE` writer - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
pub type CLKSOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTFLAG` reader - Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
pub type COUNTFLAG_R = crate::BitReader;
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
    pub fn enable(&mut self) -> ENABLE_W<SYST_CSR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<SYST_CSR_SPEC> {
        TICKINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<SYST_CSR_SPEC> {
        CLKSOURCE_W::new(self, 2)
    }
}
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features.  

You can [`read`](crate::Reg::read) this register and get [`syst_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_CSR_SPEC;
impl crate::RegisterSpec for SYST_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_csr::R`](R) reader structure"]
impl crate::Readable for SYST_CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syst_csr::W`](W) writer structure"]
impl crate::Writable for SYST_CSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CSR to value 0"]
impl crate::Resettable for SYST_CSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
