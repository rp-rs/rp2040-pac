#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `TS_EN` reader - Power on temperature sensor. 1 - enabled. 0 - disabled."]
pub type TS_EN_R = crate::BitReader<bool>;
#[doc = "Field `TS_EN` writer - Power on temperature sensor. 1 - enabled. 0 - disabled."]
pub type TS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `START_ONCE` reader - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
pub type START_ONCE_R = crate::BitReader<bool>;
#[doc = "Field `START_ONCE` writer - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
pub type START_ONCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `START_MANY` reader - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
pub type START_MANY_R = crate::BitReader<bool>;
#[doc = "Field `START_MANY` writer - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
pub type START_MANY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `READY` reader - 1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed.  
 0 whilst conversion in progress."]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `ERR` reader - The most recent ADC conversion encountered an error; result is undefined or noisy."]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR_STICKY` reader - Some past ADC conversion encountered an error. Write 1 to clear."]
pub type ERR_STICKY_R = crate::BitReader<bool>;
#[doc = "Field `ERR_STICKY` writer - Some past ADC conversion encountered an error. Write 1 to clear."]
pub type ERR_STICKY_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `AINSEL` reader - Select analog mux input. Updated automatically in round-robin mode."]
pub type AINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AINSEL` writer - Select analog mux input. Updated automatically in round-robin mode."]
pub type AINSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, u8, 3, O>;
#[doc = "Field `RROBIN` reader - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
pub type RROBIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RROBIN` writer - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
pub type RROBIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub fn start_once(&self) -> START_ONCE_R {
        START_ONCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub fn start_many(&self) -> START_MANY_R {
        START_MANY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed.  
 0 whilst conversion in progress."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The most recent ADC conversion encountered an error; result is undefined or noisy."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub fn err_sticky(&self) -> ERR_STICKY_R {
        ERR_STICKY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Select analog mux input. Updated automatically in round-robin mode."]
    #[inline(always)]
    pub fn ainsel(&self) -> AINSEL_R {
        AINSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub fn rrobin(&self) -> RROBIN_R {
        RROBIN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ts_en(&mut self) -> TS_EN_W<1> {
        TS_EN_W::new(self)
    }
    #[doc = "Bit 2 - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn start_once(&mut self) -> START_ONCE_W<2> {
        START_ONCE_W::new(self)
    }
    #[doc = "Bit 3 - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    #[must_use]
    pub fn start_many(&mut self) -> START_MANY_W<3> {
        START_MANY_W::new(self)
    }
    #[doc = "Bit 10 - Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn err_sticky(&mut self) -> ERR_STICKY_W<10> {
        ERR_STICKY_W::new(self)
    }
    #[doc = "Bits 12:14 - Select analog mux input. Updated automatically in round-robin mode."]
    #[inline(always)]
    #[must_use]
    pub fn ainsel(&mut self) -> AINSEL_W<12> {
        AINSEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    #[must_use]
    pub fn rrobin(&mut self) -> RROBIN_W<16> {
        RROBIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control and Status  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0400;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
