#[doc = "Reader of register CLK_REF_CTRL"]
pub type R = crate::R<u32, super::CLK_REF_CTRL>;
#[doc = "Writer for register CLK_REF_CTRL"]
pub type W = crate::W<u32, super::CLK_REF_CTRL>;
#[doc = "Register CLK_REF_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_REF_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the auxiliary clock source, will glitch when switching\n\nValue on reset: 0"]
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
#[doc = "Reader of field `AUXSRC`"]
pub type AUXSRC_R = crate::R<u8, AUXSRC_A>;
impl AUXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUXSRC_A::CLKSRC_PLL_USB),
            1 => Val(AUXSRC_A::CLKSRC_GPIN0),
            2 => Val(AUXSRC_A::CLKSRC_GPIN1),
            i => Res(i),
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Selects the clock source glitchlessly, can be changed on-the-fly\n\nValue on reset: 0"]
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
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::ROSC_CLKSRC_PH),
            1 => Val(SRC_A::CLKSRC_CLK_REF_AUX),
            2 => Val(SRC_A::XOSC_CLKSRC),
            i => Res(i),
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
#[doc = "Write proxy for field `SRC`"]
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
}
