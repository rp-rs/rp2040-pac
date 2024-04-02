#[doc = "Register `CLK_SYS_CTRL` reader"]
pub type R = crate::R<CLK_SYS_CTRL_SPEC>;
#[doc = "Register `CLK_SYS_CTRL` writer"]
pub type W = crate::W<CLK_SYS_CTRL_SPEC>;
#[doc = "Selects the clock source glitchlessly, can be changed on-the-fly  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `SRC` reader - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub type SRC_R = crate::BitReader<SRC_A>;
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::CLK_REF,
            true => SRC_A::CLKSRC_CLK_SYS_AUX,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_clk_ref(&self) -> bool {
        *self == SRC_A::CLK_REF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clksrc_clk_sys_aux(&self) -> bool {
        *self == SRC_A::CLKSRC_CLK_SYS_AUX
    }
}
#[doc = "Field `SRC` writer - Selects the clock source glitchlessly, can be changed on-the-fly"]
pub type SRC_W<'a, REG> = crate::BitWriter<'a, REG, SRC_A>;
impl<'a, REG> SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::CLK_REF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_clk_sys_aux(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::CLKSRC_CLK_SYS_AUX)
    }
}
#[doc = "Selects the auxiliary clock source, will glitch when switching  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for AUXSRC_A {
    type Ux = u8;
}
impl crate::IsEnum for AUXSRC_A {}
#[doc = "Field `AUXSRC` reader - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_R = crate::FieldReader<AUXSRC_A>;
impl AUXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AUXSRC_A> {
        match self.bits {
            0 => Some(AUXSRC_A::CLKSRC_PLL_SYS),
            1 => Some(AUXSRC_A::CLKSRC_PLL_USB),
            2 => Some(AUXSRC_A::ROSC_CLKSRC),
            3 => Some(AUXSRC_A::XOSC_CLKSRC),
            4 => Some(AUXSRC_A::CLKSRC_GPIN0),
            5 => Some(AUXSRC_A::CLKSRC_GPIN1),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_clksrc_pll_sys(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_SYS
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_USB
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rosc_clksrc(&self) -> bool {
        *self == AUXSRC_A::ROSC_CLKSRC
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        *self == AUXSRC_A::XOSC_CLKSRC
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_clksrc_gpin0(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_GPIN0
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_clksrc_gpin1(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_GPIN1
    }
}
#[doc = "Field `AUXSRC` writer - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AUXSRC_A>;
impl<'a, REG> AUXSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clksrc_pll_sys(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_PLL_SYS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_pll_usb(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_PLL_USB)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rosc_clksrc(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::ROSC_CLKSRC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn xosc_clksrc(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::XOSC_CLKSRC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clksrc_gpin0(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_GPIN0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clksrc_gpin1(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_GPIN1)
    }
}
impl R {
    #[doc = "Bit 0 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<CLK_SYS_CTRL_SPEC> {
        SRC_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    #[must_use]
    pub fn auxsrc(&mut self) -> AUXSRC_W<CLK_SYS_CTRL_SPEC> {
        AUXSRC_W::new(self, 5)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::Reg::read) this register and get [`clk_sys_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sys_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SYS_CTRL_SPEC;
impl crate::RegisterSpec for CLK_SYS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sys_ctrl::R`](R) reader structure"]
impl crate::Readable for CLK_SYS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_sys_ctrl::W`](W) writer structure"]
impl crate::Writable for CLK_SYS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_SYS_CTRL to value 0"]
impl crate::Resettable for CLK_SYS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
