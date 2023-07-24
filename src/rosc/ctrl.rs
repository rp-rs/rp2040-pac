#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ_RANGE` reader - Controls the number of delay stages in the ROSC ring  
 LOW uses stages 0 to 7  
 MEDIUM uses stages 0 to 5  
 HIGH uses stages 0 to 3  
 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications  
 The clock output will not glitch when changing the range up one step at a time  
 The clock output will glitch when changing the range down  
 Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
pub type FREQ_RANGE_R = crate::FieldReader<u16, FREQ_RANGE_A>;
#[doc = "Controls the number of delay stages in the ROSC ring  
 LOW uses stages 0 to 7  
 MEDIUM uses stages 0 to 5  
 HIGH uses stages 0 to 3  
 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications  
 The clock output will not glitch when changing the range up one step at a time  
 The clock output will glitch when changing the range down  
 Note: the values here are gray coded which is why HIGH comes before TOOHIGH  

Value on reset: 2720"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FREQ_RANGE_A {
    #[doc = "4004: `111110100100`"]
    LOW = 4004,
    #[doc = "4005: `111110100101`"]
    MEDIUM = 4005,
    #[doc = "4007: `111110100111`"]
    HIGH = 4007,
    #[doc = "4006: `111110100110`"]
    TOOHIGH = 4006,
}
impl From<FREQ_RANGE_A> for u16 {
    #[inline(always)]
    fn from(variant: FREQ_RANGE_A) -> Self {
        variant as _
    }
}
impl FREQ_RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_RANGE_A> {
        match self.bits {
            4004 => Some(FREQ_RANGE_A::LOW),
            4005 => Some(FREQ_RANGE_A::MEDIUM),
            4007 => Some(FREQ_RANGE_A::HIGH),
            4006 => Some(FREQ_RANGE_A::TOOHIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FREQ_RANGE_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == FREQ_RANGE_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FREQ_RANGE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOOHIGH`"]
    #[inline(always)]
    pub fn is_toohigh(&self) -> bool {
        *self == FREQ_RANGE_A::TOOHIGH
    }
}
#[doc = "Field `FREQ_RANGE` writer - Controls the number of delay stages in the ROSC ring  
 LOW uses stages 0 to 7  
 MEDIUM uses stages 0 to 5  
 HIGH uses stages 0 to 3  
 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications  
 The clock output will not glitch when changing the range up one step at a time  
 The clock output will glitch when changing the range down  
 Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
pub type FREQ_RANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u16, FREQ_RANGE_A, 12, O>;
impl<'a, const O: u8> FREQ_RANGE_W<'a, O> {
    #[doc = "`111110100100`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::LOW)
    }
    #[doc = "`111110100101`"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::MEDIUM)
    }
    #[doc = "`111110100111`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::HIGH)
    }
    #[doc = "`111110100110`"]
    #[inline(always)]
    pub fn toohigh(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::TOOHIGH)
    }
}
#[doc = "Field `ENABLE` reader - On power-up this field is initialised to ENABLE  
 The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
pub type ENABLE_R = crate::FieldReader<u16, ENABLE_A>;
#[doc = "On power-up this field is initialised to ENABLE  
 The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ENABLE_A {
    #[doc = "3358: `110100011110`"]
    DISABLE = 3358,
    #[doc = "4011: `111110101011`"]
    ENABLE = 4011,
}
impl From<ENABLE_A> for u16 {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as _
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_A> {
        match self.bits {
            3358 => Some(ENABLE_A::DISABLE),
            4011 => Some(ENABLE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Field `ENABLE` writer - On power-up this field is initialised to ENABLE  
 The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
pub type ENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, ENABLE_A, 12, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "`110100011110`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "`111110101011`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:11 - Controls the number of delay stages in the ROSC ring  
 LOW uses stages 0 to 7  
 MEDIUM uses stages 0 to 5  
 HIGH uses stages 0 to 3  
 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications  
 The clock output will not glitch when changing the range up one step at a time  
 The clock output will glitch when changing the range down  
 Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - On power-up this field is initialised to ENABLE  
 The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Controls the number of delay stages in the ROSC ring  
 LOW uses stages 0 to 7  
 MEDIUM uses stages 0 to 5  
 HIGH uses stages 0 to 3  
 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications  
 The clock output will not glitch when changing the range up one step at a time  
 The clock output will glitch when changing the range down  
 Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    #[must_use]
    pub fn freq_range(&mut self) -> FREQ_RANGE_W<0> {
        FREQ_RANGE_W::new(self)
    }
    #[doc = "Bits 12:23 - On power-up this field is initialised to ENABLE  
 The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<12> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring Oscillator control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0aa0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0aa0;
}
