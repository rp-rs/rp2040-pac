#[doc = "Register `CLK_REF_CTRL` reader"]
pub type R = crate::R<CLK_REF_CTRL_SPEC>;
#[doc = "Register `CLK_REF_CTRL` writer"]
pub type W = crate::W<CLK_REF_CTRL_SPEC>;
#[doc = "Field `SRC` reader - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub type SRC_R = crate::FieldReader<SRC_A>;
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
impl crate::FieldSpec for SRC_A {
    type Ux = u8;
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::ROSC_CLKSRC_PH),
            1 => Some(SRC_A::CLKSRC_CLK_REF_AUX),
            2 => Some(SRC_A::XOSC_CLKSRC),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_rosc_clksrc_ph(&self) -> bool {
        *self == SRC_A::ROSC_CLKSRC_PH
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clksrc_clk_ref_aux(&self) -> bool {
        *self == SRC_A::CLKSRC_CLK_REF_AUX
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        *self == SRC_A::XOSC_CLKSRC
    }
}
#[doc = "Field `SRC` writer - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub type SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SRC_A>;
impl<'a, REG, const O: u8> SRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rosc_clksrc_ph(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::ROSC_CLKSRC_PH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_clk_ref_aux(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::CLKSRC_CLK_REF_AUX)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn xosc_clksrc(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::XOSC_CLKSRC)
    }
}
#[doc = "Field `AUXSRC` reader - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_R = crate::FieldReader<AUXSRC_A>;
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
impl crate::FieldSpec for AUXSRC_A {
    type Ux = u8;
}
impl AUXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AUXSRC_A> {
        match self.bits {
            0 => Some(AUXSRC_A::CLKSRC_PLL_USB),
            1 => Some(AUXSRC_A::CLKSRC_GPIN0),
            2 => Some(AUXSRC_A::CLKSRC_GPIN1),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_USB
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clksrc_gpin0(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_GPIN0
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_clksrc_gpin1(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_GPIN1
    }
}
#[doc = "Field `AUXSRC` writer - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, AUXSRC_A>;
impl<'a, REG, const O: u8> AUXSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clksrc_pll_usb(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_PLL_USB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_gpin0(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_GPIN0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clksrc_gpin1(self) -> &'a mut crate::W<REG> {
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
    pub fn src(&mut self) -> SRC_W<CLK_REF_CTRL_SPEC, 0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 5:6 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    #[must_use]
    pub fn auxsrc(&mut self) -> AUXSRC_W<CLK_REF_CTRL_SPEC, 5> {
        AUXSRC_W::new(self)
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
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_ref_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_REF_CTRL_SPEC;
impl crate::RegisterSpec for CLK_REF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ref_ctrl::R`](R) reader structure"]
impl crate::Readable for CLK_REF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_ref_ctrl::W`](W) writer structure"]
impl crate::Writable for CLK_REF_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_REF_CTRL to value 0"]
impl crate::Resettable for CLK_REF_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
