#[doc = "Reader of register CLK_SYS_CTRL"]
pub type R = crate::R<u32, super::CLK_SYS_CTRL>;
#[doc = "Writer for register CLK_SYS_CTRL"]
pub type W = crate::W<u32, super::CLK_SYS_CTRL>;
#[doc = "Register CLK_SYS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_SYS_CTRL {
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
    CLKSRC_PLL_SYS = 0,
    #[doc = "1: `1`"]
    CLKSRC_PLL_USB = 1,
    #[doc = "2: `10`"]
    ROSC_CLKSRC = 2,
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
#[doc = "Reader of field `AUXSRC`"]
pub type AUXSRC_R = crate::R<u8, AUXSRC_A>;
impl AUXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUXSRC_A::CLKSRC_PLL_SYS),
            1 => Val(AUXSRC_A::CLKSRC_PLL_USB),
            2 => Val(AUXSRC_A::ROSC_CLKSRC),
            3 => Val(AUXSRC_A::XOSC_CLKSRC),
            4 => Val(AUXSRC_A::CLKSRC_GPIN0),
            5 => Val(AUXSRC_A::CLKSRC_GPIN1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_SYS`"]
    #[inline(always)]
    pub fn is_clksrc_pll_sys(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_SYS
    }
    #[doc = "Checks if the value of the field is `CLKSRC_PLL_USB`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_USB
    }
    #[doc = "Checks if the value of the field is `ROSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_rosc_clksrc(&self) -> bool {
        *self == AUXSRC_A::ROSC_CLKSRC
    }
    #[doc = "Checks if the value of the field is `XOSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        *self == AUXSRC_A::XOSC_CLKSRC
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
    pub fn clksrc_pll_sys(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_PLL_SYS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_pll_usb(self) -> &'a mut W {
        self.variant(AUXSRC_A::CLKSRC_PLL_USB)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rosc_clksrc(self) -> &'a mut W {
        self.variant(AUXSRC_A::ROSC_CLKSRC)
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
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Selects the clock source glitchlessly, can be changed on-the-fly\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: `0`"]
    CLK_REF = 0,
    #[doc = "1: `1`"]
    CLKSRC_CLK_SYS_AUX = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::CLK_REF,
            true => SRC_A::CLKSRC_CLK_SYS_AUX,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_REF`"]
    #[inline(always)]
    pub fn is_clk_ref(&self) -> bool {
        *self == SRC_A::CLK_REF
    }
    #[doc = "Checks if the value of the field is `CLKSRC_CLK_SYS_AUX`"]
    #[inline(always)]
    pub fn is_clksrc_clk_sys_aux(&self) -> bool {
        *self == SRC_A::CLKSRC_CLK_SYS_AUX
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk_ref(self) -> &'a mut W {
        self.variant(SRC_A::CLK_REF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_clk_sys_aux(self) -> &'a mut W {
        self.variant(SRC_A::CLKSRC_CLK_SYS_AUX)
    }
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
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&mut self) -> AUXSRC_W {
        AUXSRC_W { w: self }
    }
    #[doc = "Bit 0 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
