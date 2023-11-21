#[doc = "Register `DBGPAUSE` reader"]
pub type R = crate::R<DBGPAUSE_SPEC>;
#[doc = "Register `DBGPAUSE` writer"]
pub type W = crate::W<DBGPAUSE_SPEC>;
#[doc = "Field `DBG0` reader - Pause when processor 0 is in debug mode"]
pub type DBG0_R = crate::BitReader;
#[doc = "Field `DBG0` writer - Pause when processor 0 is in debug mode"]
pub type DBG0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG1` reader - Pause when processor 1 is in debug mode"]
pub type DBG1_R = crate::BitReader;
#[doc = "Field `DBG1` writer - Pause when processor 1 is in debug mode"]
pub type DBG1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn dbg0(&self) -> DBG0_R {
        DBG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn dbg1(&self) -> DBG1_R {
        DBG1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Pause when processor 0 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg0(&mut self) -> DBG0_W<DBGPAUSE_SPEC, 1> {
        DBG0_W::new(self)
    }
    #[doc = "Bit 2 - Pause when processor 1 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg1(&mut self) -> DBG1_W<DBGPAUSE_SPEC, 2> {
        DBG1_W::new(self)
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
#[doc = "Set bits high to enable pause when the corresponding debug ports are active  

You can [`read`](crate::generic::Reg::read) this register and get [`dbgpause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgpause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGPAUSE_SPEC;
impl crate::RegisterSpec for DBGPAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgpause::R`](R) reader structure"]
impl crate::Readable for DBGPAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgpause::W`](W) writer structure"]
impl crate::Writable for DBGPAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGPAUSE to value 0x07"]
impl crate::Resettable for DBGPAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
