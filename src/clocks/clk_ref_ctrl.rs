#[doc = "Register `CLK_REF_CTRL` reader"]
pub struct R(crate::R<CLK_REF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REF_CTRL` writer"]
pub struct W(crate::W<CLK_REF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REF_CTRL_SPEC>;
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
impl From<crate::W<CLK_REF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REF_CTRL_SPEC>) -> Self {
        W(writer)
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
    CLKSRC_GPIN0 = 1,
    #[doc = "2: `10`"]
    CLKSRC_GPIN1 = 2,
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
            1 => Some(AUXSRC_A::CLKSRC_GPIN0),
            2 => Some(AUXSRC_A::CLKSRC_GPIN1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_USB`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        **self == AUXSRC_A::CLKSRC_PLL_USB
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
    pub fn clksrc_gpin0(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_GPIN0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clksrc_gpin1(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_GPIN1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Selects the clock source glitchlessly, can be changed on-the-fly  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: `0`"]
    ROSC_CLKSRC_PH = 0,
    #[doc = "1: `1`"]
    CLKSRC_CLK_REF_AUX = 1,
    #[doc = "2: `10`"]
    XOSC_CLKSRC = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::ROSC_CLKSRC_PH),
            1 => Some(SRC_A::CLKSRC_CLK_REF_AUX),
            2 => Some(SRC_A::XOSC_CLKSRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ROSC_CLKSRC_PH`"]
    #[inline(always)]
    pub fn is_rosc_clksrc_ph(&self) -> bool {
        **self == SRC_A::ROSC_CLKSRC_PH
    }
    #[doc = "Checks if the value of the field is `CLKSRC_CLK_REF_AUX`"]
    #[inline(always)]
    pub fn is_clksrc_clk_ref_aux(&self) -> bool {
        **self == SRC_A::CLKSRC_CLK_REF_AUX
    }
    #[doc = "Checks if the value of the field is `XOSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        **self == SRC_A::XOSC_CLKSRC
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rosc_clksrc_ph(self) -> &'a mut W {
        self.variant(SRC_A::ROSC_CLKSRC_PH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_clk_ref_aux(self) -> &'a mut W {
        self.variant(SRC_A::CLKSRC_CLK_REF_AUX)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn xosc_clksrc(self) -> &'a mut W {
        self.variant(SRC_A::XOSC_CLKSRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:6 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 5:6 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&mut self) -> AUXSRC_W {
        AUXSRC_W { w: self }
    }
    #[doc = "Bits 0:1 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
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

For information about available fields see [clk_ref_ctrl](index.html) module"]
pub struct CLK_REF_CTRL_SPEC;
impl crate::RegisterSpec for CLK_REF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ref_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_REF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ref_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_REF_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REF_CTRL to value 0"]
impl crate::Resettable for CLK_REF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
