#[doc = "Register `CLK_GPOUT2_CTRL` reader"]
pub struct R(crate::R<CLK_GPOUT2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GPOUT2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GPOUT2_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GPOUT2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GPOUT2_CTRL` writer"]
pub struct W(crate::W<CLK_GPOUT2_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GPOUT2_CTRL_SPEC>;
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
impl From<crate::W<CLK_GPOUT2_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GPOUT2_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXSRC` reader - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_R = crate::FieldReader<u8, AUXSRC_A>;
#[doc = "Selects the auxiliary clock source, will glitch when switching  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl AUXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUXSRC_A> {
        match self.bits {
            0 => Some(AUXSRC_A::CLKSRC_PLL_SYS),
            1 => Some(AUXSRC_A::CLKSRC_GPIN0),
            2 => Some(AUXSRC_A::CLKSRC_GPIN1),
            3 => Some(AUXSRC_A::CLKSRC_PLL_USB),
            4 => Some(AUXSRC_A::ROSC_CLKSRC_PH),
            5 => Some(AUXSRC_A::XOSC_CLKSRC),
            6 => Some(AUXSRC_A::CLK_SYS),
            7 => Some(AUXSRC_A::CLK_USB),
            8 => Some(AUXSRC_A::CLK_ADC),
            9 => Some(AUXSRC_A::CLK_RTC),
            10 => Some(AUXSRC_A::CLK_REF),
            _ => None,
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
#[doc = "Field `AUXSRC` writer - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_GPOUT2_CTRL_SPEC, u8, AUXSRC_A, 4, O>;
impl<'a, const O: u8> AUXSRC_W<'a, O> {
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
}
#[doc = "Field `KILL` reader - Asynchronously kills the clock generator"]
pub type KILL_R = crate::BitReader<bool>;
#[doc = "Field `KILL` writer - Asynchronously kills the clock generator"]
pub type KILL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GPOUT2_CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Starts and stops the clock generator cleanly"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Starts and stops the clock generator cleanly"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GPOUT2_CTRL_SPEC, bool, O>;
#[doc = "Field `DC50` reader - Enables duty cycle correction for odd divisors"]
pub type DC50_R = crate::BitReader<bool>;
#[doc = "Field `DC50` writer - Enables duty cycle correction for odd divisors"]
pub type DC50_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GPOUT2_CTRL_SPEC, bool, O>;
#[doc = "Field `PHASE` reader - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
pub type PHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHASE` writer - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
pub type PHASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_GPOUT2_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `NUDGE` reader - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
pub type NUDGE_R = crate::BitReader<bool>;
#[doc = "Field `NUDGE` writer - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
pub type NUDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GPOUT2_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 5:8 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Asynchronously kills the clock generator"]
    #[inline(always)]
    pub fn kill(&self) -> KILL_R {
        KILL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables duty cycle correction for odd divisors"]
    #[inline(always)]
    pub fn dc50(&self) -> DC50_R {
        DC50_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
    #[inline(always)]
    pub fn nudge(&self) -> NUDGE_R {
        NUDGE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:8 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    #[must_use]
    pub fn auxsrc(&mut self) -> AUXSRC_W<5> {
        AUXSRC_W::new(self)
    }
    #[doc = "Bit 10 - Asynchronously kills the clock generator"]
    #[inline(always)]
    #[must_use]
    pub fn kill(&mut self) -> KILL_W<10> {
        KILL_W::new(self)
    }
    #[doc = "Bit 11 - Starts and stops the clock generator cleanly"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<11> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 12 - Enables duty cycle correction for odd divisors"]
    #[inline(always)]
    #[must_use]
    pub fn dc50(&mut self) -> DC50_W<12> {
        DC50_W::new(self)
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PHASE_W<16> {
        PHASE_W::new(self)
    }
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
    #[inline(always)]
    #[must_use]
    pub fn nudge(&mut self) -> NUDGE_W<20> {
        NUDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_gpout2_ctrl](index.html) module"]
pub struct CLK_GPOUT2_CTRL_SPEC;
impl crate::RegisterSpec for CLK_GPOUT2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gpout2_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_GPOUT2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gpout2_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_GPOUT2_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_GPOUT2_CTRL to value 0"]
impl crate::Resettable for CLK_GPOUT2_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
