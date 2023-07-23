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
#[doc = "Field `SRC` reader - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "Selects the clock source glitchlessly, can be changed on-the-fly  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SRC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SRC_A::ROSC_CLKSRC_PH
    }
    #[doc = "Checks if the value of the field is `CLKSRC_CLK_REF_AUX`"]
    #[inline(always)]
    pub fn is_clksrc_clk_ref_aux(&self) -> bool {
        *self == SRC_A::CLKSRC_CLK_REF_AUX
    }
    #[doc = "Checks if the value of the field is `XOSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        *self == SRC_A::XOSC_CLKSRC
    }
}
#[doc = "Field `SRC` writer - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_REF_CTRL_SPEC, u8, SRC_A, 2, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
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
}
#[doc = "Field `AUXSRC` reader - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_R = crate::FieldReader<u8, AUXSRC_A>;
#[doc = "Selects the auxiliary clock source, will glitch when switching  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl AUXSRC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == AUXSRC_A::CLKSRC_PLL_USB
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
}
#[doc = "Field `AUXSRC` writer - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_REF_CTRL_SPEC, u8, AUXSRC_A, 2, O>;
impl<'a, const O: u8> AUXSRC_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 5:6 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 5:6 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    #[must_use]
    pub fn auxsrc(&mut self) -> AUXSRC_W<5> {
        AUXSRC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_REF_CTRL to value 0"]
impl crate::Resettable for CLK_REF_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
