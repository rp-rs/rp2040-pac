#[doc = "Register `CLK_ADC_CTRL` reader"]
pub struct R(crate::R<CLK_ADC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ADC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ADC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ADC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_ADC_CTRL` writer"]
pub struct W(crate::W<CLK_ADC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_ADC_CTRL_SPEC>;
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
impl From<crate::W<CLK_ADC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_ADC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUDGE` reader - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
pub struct NUDGE_R(crate::FieldReader<bool, bool>);
impl NUDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NUDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUDGE` writer - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PHASE` reader - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
pub struct PHASE_R(crate::FieldReader<u8, u8>);
impl PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE` writer - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
pub struct PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Starts and stops the clock generator cleanly"]
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
#[doc = "Field `ENABLE` writer - Starts and stops the clock generator cleanly"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `KILL` reader - Asynchronously kills the clock generator"]
pub struct KILL_R(crate::FieldReader<bool, bool>);
impl KILL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KILL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KILL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KILL` writer - Asynchronously kills the clock generator"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Selects the auxiliary clock source, will glitch when switching  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXSRC_A {
    #[doc = "0: `0`"]
    CLKSRC_PLL_USB = 0,
    #[doc = "1: `1`"]
    CLKSRC_PLL_SYS = 1,
    #[doc = "2: `10`"]
    ROSC_CLKSRC_PH = 2,
    #[doc = "3: `11`"]
    XOSC_CLKSRC = 3,
    #[doc = "4: `100`"]
    CLKSRC_GPIN0 = 4,
    #[doc = "5: `101`"]
    CLKSRC_GPIN1 = 5,
}
impl From<AUXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXSRC` reader - Selects the auxiliary clock source, will glitch when switching"]
pub struct AUXSRC_R(crate::FieldReader<u8, AUXSRC_A>);
impl AUXSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AUXSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUXSRC_A> {
        match self.bits {
            0 => Some(AUXSRC_A::CLKSRC_PLL_USB),
            1 => Some(AUXSRC_A::CLKSRC_PLL_SYS),
            2 => Some(AUXSRC_A::ROSC_CLKSRC_PH),
            3 => Some(AUXSRC_A::XOSC_CLKSRC),
            4 => Some(AUXSRC_A::CLKSRC_GPIN0),
            5 => Some(AUXSRC_A::CLKSRC_GPIN1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_USB`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        **self == AUXSRC_A::CLKSRC_PLL_USB
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_SYS`"]
    #[inline(always)]
    pub fn is_clksrc_pll_sys(&self) -> bool {
        **self == AUXSRC_A::CLKSRC_PLL_SYS
    }
    #[doc = "Checks if the value of the field is `ROSC_CLKSRC_PH`"]
    #[inline(always)]
    pub fn is_rosc_clksrc_ph(&self) -> bool {
        **self == AUXSRC_A::ROSC_CLKSRC_PH
    }
    #[doc = "Checks if the value of the field is `XOSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        **self == AUXSRC_A::XOSC_CLKSRC
    }
    #[doc = "Checks if the value of the field is `CLKSRC_GPIN0`"]
    #[inline(always)]
    pub fn is_clksrc_gpin0(&self) -> bool {
        **self == AUXSRC_A::CLKSRC_GPIN0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_GPIN1`"]
    #[inline(always)]
    pub fn is_clksrc_gpin1(&self) -> bool {
        **self == AUXSRC_A::CLKSRC_GPIN1
    }
}
impl core::ops::Deref for AUXSRC_R {
    type Target = crate::FieldReader<u8, AUXSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXSRC` writer - Selects the auxiliary clock source, will glitch when switching"]
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
    pub fn clksrc_pll_usb(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_PLL_USB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_pll_sys(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_PLL_SYS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rosc_clksrc_ph(self) -> &'a mut W {
        self.variant(AUXSRC_A::ROSC_CLKSRC_PH)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn xosc_clksrc(self) -> &'a mut W {
        self.variant(AUXSRC_A::XOSC_CLKSRC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clksrc_gpin0(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_GPIN0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clksrc_gpin1(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_GPIN1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
    #[inline(always)]
    pub fn nudge(&self) -> NUDGE_R {
        NUDGE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 16) & 0x03) as u8)
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
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock  
 This can be done at any time"]
    #[inline(always)]
    pub fn nudge(&mut self) -> NUDGE_W {
        NUDGE_W { w: self }
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock  
 This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W {
        PHASE_W { w: self }
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
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&mut self) -> AUXSRC_W {
        AUXSRC_W { w: self }
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

For information about available fields see [clk_adc_ctrl](index.html) module"]
pub struct CLK_ADC_CTRL_SPEC;
impl crate::RegisterSpec for CLK_ADC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_adc_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_ADC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_adc_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_ADC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_ADC_CTRL to value 0"]
impl crate::Resettable for CLK_ADC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
