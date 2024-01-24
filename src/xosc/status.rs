#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `FREQ_RANGE` reader - The current frequency range setting, always reads 0"]
pub type FREQ_RANGE_R = crate::FieldReader<FREQ_RANGE_A>;
#[doc = "The current frequency range setting, always reads 0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for FREQ_RANGE_A {
    type Ux = u8;
}
impl FREQ_RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FREQ_RANGE_A {
        match self.bits {
            0 => FREQ_RANGE_A::_1_15MHZ,
            1 => FREQ_RANGE_A::RESERVED_1,
            2 => FREQ_RANGE_A::RESERVED_2,
            3 => FREQ_RANGE_A::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_1_15mhz(&self) -> bool {
        *self == FREQ_RANGE_A::_1_15MHZ
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reserved_1(&self) -> bool {
        *self == FREQ_RANGE_A::RESERVED_1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == FREQ_RANGE_A::RESERVED_2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == FREQ_RANGE_A::RESERVED_3
    }
}
#[doc = "Field `ENABLED` reader - Oscillator is enabled but not necessarily running and stable, resets to 0"]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `BADWRITE` reader - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
pub type BADWRITE_R = crate::BitReader;
#[doc = "Field `BADWRITE` writer - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
pub type BADWRITE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `STABLE` reader - Oscillator is running and stable"]
pub type STABLE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - The current frequency range setting, always reads 0"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable, resets to 0"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&self) -> BADWRITE_R {
        BADWRITE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    #[must_use]
    pub fn badwrite(&mut self) -> BADWRITE_W<STATUS_SPEC> {
        BADWRITE_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Crystal Oscillator Status  

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100_0000;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
