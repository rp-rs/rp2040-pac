#[doc = "Reader of register CLK_GPOUT3_CTRL"]
pub type R = crate::R<u32, super::CLK_GPOUT3_CTRL>;
#[doc = "Writer for register CLK_GPOUT3_CTRL"]
pub type W = crate::W<u32, super::CLK_GPOUT3_CTRL>;
#[doc = "Register CLK_GPOUT3_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_GPOUT3_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NUDGE`"]
pub type NUDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NUDGE`"]
pub struct NUDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> NUDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PHASE`"]
pub type PHASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE`"]
pub struct PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DC50`"]
pub type DC50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DC50`"]
pub struct DC50_W<'a> {
    w: &'a mut W,
}
impl<'a> DC50_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `KILL`"]
pub type KILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KILL`"]
pub struct KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> KILL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Selects the auxiliary clock source, will glitch when switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXSRC_A {
    #[doc = "0: `0`"]
    CLKSRC_PLL_SYS = 0,
    #[doc = "1: `1`"]
    CLKSRC_GPIN0 = 1,
    #[doc = "2: `10`"]
    CLKSRC_GPIN1 = 2,
    #[doc = "3: `11`"]
    CLKSRC_PLL_USB = 3,
    #[doc = "4: `100`"]
    ROSC_CLKSRC_PH = 4,
    #[doc = "5: `101`"]
    XOSC_CLKSRC = 5,
    #[doc = "6: `110`"]
    CLK_SYS = 6,
    #[doc = "7: `111`"]
    CLK_USB = 7,
    #[doc = "8: `1000`"]
    CLK_ADC = 8,
    #[doc = "9: `1001`"]
    CLK_RTC = 9,
    #[doc = "10: `1010`"]
    CLK_REF = 10,
}
impl From<AUXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUXSRC`"]
pub type AUXSRC_R = crate::R<u8, AUXSRC_A>;
impl AUXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUXSRC_A::CLKSRC_PLL_SYS),
            1 => Val(AUXSRC_A::CLKSRC_GPIN0),
            2 => Val(AUXSRC_A::CLKSRC_GPIN1),
            3 => Val(AUXSRC_A::CLKSRC_PLL_USB),
            4 => Val(AUXSRC_A::ROSC_CLKSRC_PH),
            5 => Val(AUXSRC_A::XOSC_CLKSRC),
            6 => Val(AUXSRC_A::CLK_SYS),
            7 => Val(AUXSRC_A::CLK_USB),
            8 => Val(AUXSRC_A::CLK_ADC),
            9 => Val(AUXSRC_A::CLK_RTC),
            10 => Val(AUXSRC_A::CLK_REF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_SYS`"]
    #[inline(always)]
    pub fn is_clksrc_pll_sys(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_SYS
    }
    #[doc = "Checks if the value of the field is `CLKSRC_GPIN0`"]
    #[inline(always)]
    pub fn is_clksrc_gpin0(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_GPIN0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_GPIN1`"]
    #[inline(always)]
    pub fn is_clksrc_gpin1(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_GPIN1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_USB`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_USB
    }
    #[doc = "Checks if the value of the field is `ROSC_CLKSRC_PH`"]
    #[inline(always)]
    pub fn is_rosc_clksrc_ph(&self) -> bool {
        *self == AUXSRC_A::ROSC_CLKSRC_PH
    }
    #[doc = "Checks if the value of the field is `XOSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        *self == AUXSRC_A::XOSC_CLKSRC
    }
    #[doc = "Checks if the value of the field is `CLK_SYS`"]
    #[inline(always)]
    pub fn is_clk_sys(&self) -> bool {
        *self == AUXSRC_A::CLK_SYS
    }
    #[doc = "Checks if the value of the field is `CLK_USB`"]
    #[inline(always)]
    pub fn is_clk_usb(&self) -> bool {
        *self == AUXSRC_A::CLK_USB
    }
    #[doc = "Checks if the value of the field is `CLK_ADC`"]
    #[inline(always)]
    pub fn is_clk_adc(&self) -> bool {
        *self == AUXSRC_A::CLK_ADC
    }
    #[doc = "Checks if the value of the field is `CLK_RTC`"]
    #[inline(always)]
    pub fn is_clk_rtc(&self) -> bool {
        *self == AUXSRC_A::CLK_RTC
    }
    #[doc = "Checks if the value of the field is `CLK_REF`"]
    #[inline(always)]
    pub fn is_clk_ref(&self) -> bool {
        *self == AUXSRC_A::CLK_REF
    }
}
#[doc = "Write proxy for field `AUXSRC`"]
pub struct AUXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clksrc_pll_sys(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_PLL_SYS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_gpin0(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_GPIN0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clksrc_gpin1(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_GPIN1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn clksrc_pll_usb(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_PLL_USB)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rosc_clksrc_ph(self) -> &'a mut W {
        self.variant(AUXSRC_A::ROSC_CLKSRC_PH)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn xosc_clksrc(self) -> &'a mut W {
        self.variant(AUXSRC_A::XOSC_CLKSRC)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clk_sys(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLK_SYS)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn clk_usb(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLK_USB)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn clk_adc(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLK_ADC)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn clk_rtc(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLK_RTC)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn clk_ref(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLK_REF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock\\n This can be done at any time"]
    #[inline(always)]
    pub fn nudge(&self) -> NUDGE_R {
        NUDGE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock\\n This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Enables duty cycle correction for odd divisors"]
    #[inline(always)]
    pub fn dc50(&self) -> DC50_R {
        DC50_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Asynchronously kills the clock generator"]
    #[inline(always)]
    pub fn kill(&self) -> KILL_R {
        KILL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock\\n This can be done at any time"]
    #[inline(always)]
    pub fn nudge(&mut self) -> NUDGE_W {
        NUDGE_W { w: self }
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock\\n This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W {
        PHASE_W { w: self }
    }
    #[doc = "Bit 12 - Enables duty cycle correction for odd divisors"]
    #[inline(always)]
    pub fn dc50(&mut self) -> DC50_W {
        DC50_W { w: self }
    }
    #[doc = "Bit 11 - Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 10 - Asynchronously kills the clock generator"]
    #[inline(always)]
    pub fn kill(&mut self) -> KILL_W {
        KILL_W { w: self }
    }
    #[doc = "Bits 5:8 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&mut self) -> AUXSRC_W {
        AUXSRC_W { w: self }
    }
}
