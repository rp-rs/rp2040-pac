#[doc = "Register `CLK_RTC_CTRL` reader"]
pub type R = crate::R<CLK_RTC_CTRL_SPEC>;
#[doc = "Register `CLK_RTC_CTRL` writer"]
pub type W = crate::W<CLK_RTC_CTRL_SPEC>;
#[doc = "Selects the auxiliary clock source, will glitch when switching  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for AUXSRC_A {
    type Ux = u8;
}
#[doc = "Field `AUXSRC` reader - Selects the auxiliary clock source, will glitch when switching"]
pub type AUXSRC_R = crate::FieldReader<AUXSRC_A>;
impl AUXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AUXSRC_A> {
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_USB
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clksrc_pll_sys(&self) -> bool {
        *self == AUXSRC_A::CLKSRC_PLL_SYS
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rosc_clksrc_ph(&self) -> bool {
        *self == AUXSRC_A::ROSC_CLKSRC_PH
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
    pub fn clksrc_pll_usb(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_PLL_USB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_pll_sys(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::CLKSRC_PLL_SYS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rosc_clksrc_ph(self) -> &'a mut crate::W<REG> {
        self.variant(AUXSRC_A::ROSC_CLKSRC_PH)
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
#[doc = "Field `KILL` reader - Asynchronously kills the clock generator"]
pub type KILL_R = crate::BitReader;
#[doc = "Field `KILL` writer - Asynchronously kills the clock generator"]
pub type KILL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Starts and stops the clock generator cleanly"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Starts and stops the clock generator cleanly"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASE` reader - This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
pub type PHASE_R = crate::FieldReader;
#[doc = "Field `PHASE` writer - This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
pub type PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUDGE` reader - An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
pub type NUDGE_R = crate::BitReader;
#[doc = "Field `NUDGE` writer - An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
pub type NUDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn auxsrc(&self) -> AUXSRC_R {
        AUXSRC_R::new(((self.bits >> 5) & 7) as u8)
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
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub fn nudge(&self) -> NUDGE_R {
        NUDGE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    #[must_use]
    pub fn auxsrc(&mut self) -> AUXSRC_W<CLK_RTC_CTRL_SPEC> {
        AUXSRC_W::new(self, 5)
    }
    #[doc = "Bit 10 - Asynchronously kills the clock generator"]
    #[inline(always)]
    #[must_use]
    pub fn kill(&mut self) -> KILL_W<CLK_RTC_CTRL_SPEC> {
        KILL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Starts and stops the clock generator cleanly"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_RTC_CTRL_SPEC> {
        ENABLE_W::new(self, 11)
    }
    #[doc = "Bits 16:17 - This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PHASE_W<CLK_RTC_CTRL_SPEC> {
        PHASE_W::new(self, 16)
    }
    #[doc = "Bit 20 - An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    #[must_use]
    pub fn nudge(&mut self) -> NUDGE_W<CLK_RTC_CTRL_SPEC> {
        NUDGE_W::new(self, 20)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_RTC_CTRL_SPEC;
impl crate::RegisterSpec for CLK_RTC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_rtc_ctrl::R`](R) reader structure"]
impl crate::Readable for CLK_RTC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_rtc_ctrl::W`](W) writer structure"]
impl crate::Writable for CLK_RTC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_RTC_CTRL to value 0"]
impl crate::Resettable for CLK_RTC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
