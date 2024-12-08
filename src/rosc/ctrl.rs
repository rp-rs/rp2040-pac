#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH  

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
impl crate::FieldSpec for FREQ_RANGE_A {
    type Ux = u16;
}
#[doc = "Field `FREQ_RANGE` reader - Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
pub type FREQ_RANGE_R = crate::FieldReader<FREQ_RANGE_A>;
impl FREQ_RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FREQ_RANGE_A> {
        match self.bits {
            4004 => Some(FREQ_RANGE_A::LOW),
            4005 => Some(FREQ_RANGE_A::MEDIUM),
            4007 => Some(FREQ_RANGE_A::HIGH),
            4006 => Some(FREQ_RANGE_A::TOOHIGH),
            _ => None,
        }
    }
    #[doc = "`111110100100`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FREQ_RANGE_A::LOW
    }
    #[doc = "`111110100101`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == FREQ_RANGE_A::MEDIUM
    }
    #[doc = "`111110100111`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FREQ_RANGE_A::HIGH
    }
    #[doc = "`111110100110`"]
    #[inline(always)]
    pub fn is_toohigh(&self) -> bool {
        *self == FREQ_RANGE_A::TOOHIGH
    }
}
#[doc = "Field `FREQ_RANGE` writer - Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
pub type FREQ_RANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, FREQ_RANGE_A>;
impl<'a, REG> FREQ_RANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`111110100100`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_RANGE_A::LOW)
    }
    #[doc = "`111110100101`"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_RANGE_A::MEDIUM)
    }
    #[doc = "`111110100111`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_RANGE_A::HIGH)
    }
    #[doc = "`111110100110`"]
    #[inline(always)]
    pub fn toohigh(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_RANGE_A::TOOHIGH)
    }
}
#[doc = "On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.  

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
impl crate::FieldSpec for ENABLE_A {
    type Ux = u16;
}
#[doc = "Field `ENABLE` reader - On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
pub type ENABLE_R = crate::FieldReader<ENABLE_A>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ENABLE_A> {
        match self.bits {
            3358 => Some(ENABLE_A::DISABLE),
            4011 => Some(ENABLE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "`110100011110`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "`111110101011`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Field `ENABLE` writer - On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
pub type ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`110100011110`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "`111110101011`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:11 - Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    #[must_use]
    pub fn freq_range(&mut self) -> FREQ_RANGE_W<CTRL_SPEC> {
        FREQ_RANGE_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRL_SPEC> {
        ENABLE_W::new(self, 12)
    }
}
#[doc = "Ring Oscillator control  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0aa0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0aa0;
}
