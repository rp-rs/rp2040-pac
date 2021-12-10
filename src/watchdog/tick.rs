#[doc = "Register `TICK` reader"]
pub struct R(crate::R<TICK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TICK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TICK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TICK` writer"]
pub struct W(crate::W<TICK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICK_SPEC>;
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
impl From<crate::W<TICK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNNING` reader - Is the tick generator running?"]
pub struct RUNNING_R(crate::FieldReader<bool, bool>);
impl RUNNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` reader - start / stop tick generation"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - start / stop tick generation"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CYCLES` reader - Total number of clk_tick cycles before the next tick."]
pub struct CYCLES_R(crate::FieldReader<u16, u16>);
impl CYCLES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCLES` writer - Total number of clk_tick cycles before the next tick."]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:19 - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 10 - Is the tick generator running?"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - start / stop tick generation"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 9 - start / stop tick generation"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the tick generator  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [tick](index.html) module"]
pub struct TICK_SPEC;
impl crate::RegisterSpec for TICK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tick::R](R) reader structure"]
impl crate::Readable for TICK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tick::W](W) writer structure"]
impl crate::Writable for TICK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TICK to value 0x0200"]
impl crate::Resettable for TICK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
