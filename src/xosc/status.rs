#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STABLE` reader - Oscillator is running and stable"]
pub struct STABLE_R(crate::FieldReader<bool, bool>);
impl STABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BADWRITE` reader - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
pub struct BADWRITE_R(crate::FieldReader<bool, bool>);
impl BADWRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BADWRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BADWRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BADWRITE` writer - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
pub struct BADWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> BADWRITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - Oscillator is enabled but not necessarily running and stable, resets to 0"]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "The current frequency range setting, always reads 0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQ_RANGE_A {
    #[doc = "0: `0`"]
    _1_15MHZ = 0,
    #[doc = "1: `1`"]
    RESERVED_1 = 1,
    #[doc = "2: `10`"]
    RESERVED_2 = 2,
    #[doc = "3: `11`"]
    RESERVED_3 = 3,
}
impl From<FREQ_RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQ_RANGE` reader - The current frequency range setting, always reads 0"]
pub struct FREQ_RANGE_R(crate::FieldReader<u8, FREQ_RANGE_A>);
impl FREQ_RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQ_RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQ_RANGE_A {
        match self.bits {
            0 => FREQ_RANGE_A::_1_15MHZ,
            1 => FREQ_RANGE_A::RESERVED_1,
            2 => FREQ_RANGE_A::RESERVED_2,
            3 => FREQ_RANGE_A::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_15MHZ`"]
    #[inline(always)]
    pub fn is_1_15mhz(&self) -> bool {
        **self == FREQ_RANGE_A::_1_15MHZ
    }
    #[doc = "Checks if the value of the field is `RESERVED_1`"]
    #[inline(always)]
    pub fn is_reserved_1(&self) -> bool {
        **self == FREQ_RANGE_A::RESERVED_1
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        **self == FREQ_RANGE_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        **self == FREQ_RANGE_A::RESERVED_3
    }
}
impl core::ops::Deref for FREQ_RANGE_R {
    type Target = crate::FieldReader<u8, FREQ_RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&self) -> BADWRITE_R {
        BADWRITE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable, resets to 0"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - The current frequency range setting, always reads 0"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&mut self) -> BADWRITE_W {
        BADWRITE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal Oscillator Status  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
