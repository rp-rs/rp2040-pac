#[doc = "Reader of register SYST_CSR"]
pub type R = crate::R<u32, super::SYST_CSR>;
#[doc = "Writer for register SYST_CSR"]
pub type W = crate::W<u32, super::SYST_CSR>;
#[doc = "Register SYST_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYST_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTFLAG`"]
pub type COUNTFLAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKSOURCE`"]
pub type CLKSOURCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKSOURCE`"]
pub struct CLKSOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSOURCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TICKINT`"]
pub type TICKINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICKINT`"]
pub struct TICKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.\\n Selects the SysTick timer clock source:\\n 0 = External reference clock.\\n 1 = Processor clock."]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables SysTick exception request:\\n 0 = Counting down to zero does not assert the SysTick exception request.\\n 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable SysTick counter:\\n 0 = Counter disabled.\\n 1 = Counter enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.\\n Selects the SysTick timer clock source:\\n 0 = External reference clock.\\n 1 = Processor clock."]
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W {
        CLKSOURCE_W { w: self }
    }
    #[doc = "Bit 1 - Enables SysTick exception request:\\n 0 = Counting down to zero does not assert the SysTick exception request.\\n 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W {
        TICKINT_W { w: self }
    }
    #[doc = "Bit 0 - Enable SysTick counter:\\n 0 = Counter disabled.\\n 1 = Counter enabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
