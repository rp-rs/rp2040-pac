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
#[doc = "Field `RROBIN` reader - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
pub struct RROBIN_R(crate::FieldReader<u8, u8>);
impl RROBIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RROBIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RROBIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RROBIN` writer - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
pub struct RROBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RROBIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `AINSEL` reader - Select analog mux input. Updated automatically in round-robin mode."]
pub struct AINSEL_R(crate::FieldReader<u8, u8>);
impl AINSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AINSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AINSEL` writer - Select analog mux input. Updated automatically in round-robin mode."]
pub struct AINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `ERR_STICKY` reader - Some past ADC conversion encountered an error. Write 1 to clear."]
pub struct ERR_STICKY_R(crate::FieldReader<bool, bool>);
impl ERR_STICKY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_STICKY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_STICKY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_STICKY` writer - Some past ADC conversion encountered an error. Write 1 to clear."]
pub struct ERR_STICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_STICKY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ERR` reader - The most recent ADC conversion encountered an error; result is undefined or noisy."]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY` reader - 1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed.  
 0 whilst conversion in progress."]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_MANY` reader - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
pub struct START_MANY_R(crate::FieldReader<bool, bool>);
impl START_MANY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_MANY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_MANY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_MANY` writer - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
pub struct START_MANY_W<'a> {
    w: &'a mut W,
}
impl<'a> START_MANY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `START_ONCE` reader - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
pub struct START_ONCE_R(crate::FieldReader<bool, bool>);
impl START_ONCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_ONCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_ONCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_ONCE` writer - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
pub struct START_ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_ONCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TS_EN` reader - Power on temperature sensor. 1 - enabled. 0 - disabled."]
pub struct TS_EN_R(crate::FieldReader<bool, bool>);
impl TS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_EN` writer - Power on temperature sensor. 1 - enabled. 0 - disabled."]
pub struct TS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub fn rrobin(&self) -> RROBIN_R {
        RROBIN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Select analog mux input. Updated automatically in round-robin mode."]
    #[inline(always)]
    pub fn ainsel(&self) -> AINSEL_R {
        AINSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub fn err_sticky(&self) -> ERR_STICKY_R {
        ERR_STICKY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The most recent ADC conversion encountered an error; result is undefined or noisy."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed.  
 0 whilst conversion in progress."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub fn start_many(&self) -> START_MANY_R {
        START_MANY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub fn start_once(&self) -> START_ONCE_R {
        START_ONCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.  
 Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.  
 The first channel to be sampled will be the one currently indicated by AINSEL.  
 AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub fn rrobin(&mut self) -> RROBIN_W {
        RROBIN_W { w: self }
    }
    #[doc = "Bits 12:14 - Select analog mux input. Updated automatically in round-robin mode."]
    #[inline(always)]
    pub fn ainsel(&mut self) -> AINSEL_W {
        AINSEL_W { w: self }
    }
    #[doc = "Bit 10 - Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub fn err_sticky(&mut self) -> ERR_STICKY_W {
        ERR_STICKY_W { w: self }
    }
    #[doc = "Bit 3 - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub fn start_many(&mut self) -> START_MANY_W {
        START_MANY_W { w: self }
    }
    #[doc = "Bit 2 - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub fn start_once(&mut self) -> START_ONCE_W {
        START_ONCE_W { w: self }
    }
    #[doc = "Bit 1 - Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn ts_en(&mut self) -> TS_EN_W {
        TS_EN_W { w: self }
    }
    #[doc = "Bit 0 - Power on ADC and enable its clock.  
 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
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
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
