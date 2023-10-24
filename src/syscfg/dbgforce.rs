#[doc = "Register `DBGFORCE` reader"]
pub type R = crate::R<DBGFORCE_SPEC>;
#[doc = "Register `DBGFORCE` writer"]
pub type W = crate::W<DBGFORCE_SPEC>;
#[doc = "Field `PROC0_SWDO` reader - Observe the value of processor 0 SWDIO output."]
pub type PROC0_SWDO_R = crate::BitReader;
#[doc = "Field `PROC0_SWDI` reader - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
pub type PROC0_SWDI_R = crate::BitReader;
#[doc = "Field `PROC0_SWDI` writer - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
pub type PROC0_SWDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROC0_SWCLK` reader - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
pub type PROC0_SWCLK_R = crate::BitReader;
#[doc = "Field `PROC0_SWCLK` writer - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
pub type PROC0_SWCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROC0_ATTACH` reader - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub type PROC0_ATTACH_R = crate::BitReader;
#[doc = "Field `PROC0_ATTACH` writer - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub type PROC0_ATTACH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROC1_SWDO` reader - Observe the value of processor 1 SWDIO output."]
pub type PROC1_SWDO_R = crate::BitReader;
#[doc = "Field `PROC1_SWDI` reader - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
pub type PROC1_SWDI_R = crate::BitReader;
#[doc = "Field `PROC1_SWDI` writer - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
pub type PROC1_SWDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROC1_SWCLK` reader - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
pub type PROC1_SWCLK_R = crate::BitReader;
#[doc = "Field `PROC1_SWCLK` writer - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
pub type PROC1_SWCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROC1_ATTACH` reader - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub type PROC1_ATTACH_R = crate::BitReader;
#[doc = "Field `PROC1_ATTACH` writer - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub type PROC1_ATTACH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Observe the value of processor 0 SWDIO output."]
    #[inline(always)]
    pub fn proc0_swdo(&self) -> PROC0_SWDO_R {
        PROC0_SWDO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swdi(&self) -> PROC0_SWDI_R {
        PROC0_SWDI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swclk(&self) -> PROC0_SWCLK_R {
        PROC0_SWCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc0_attach(&self) -> PROC0_ATTACH_R {
        PROC0_ATTACH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Observe the value of processor 1 SWDIO output."]
    #[inline(always)]
    pub fn proc1_swdo(&self) -> PROC1_SWDO_R {
        PROC1_SWDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swdi(&self) -> PROC1_SWDI_R {
        PROC1_SWDI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swclk(&self) -> PROC1_SWCLK_R {
        PROC1_SWCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc1_attach(&self) -> PROC1_ATTACH_R {
        PROC1_ATTACH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    #[must_use]
    pub fn proc0_swdi(&mut self) -> PROC0_SWDI_W<DBGFORCE_SPEC, 1> {
        PROC0_SWDI_W::new(self)
    }
    #[doc = "Bit 2 - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    #[must_use]
    pub fn proc0_swclk(&mut self) -> PROC0_SWCLK_W<DBGFORCE_SPEC, 2> {
        PROC0_SWCLK_W::new(self)
    }
    #[doc = "Bit 3 - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    #[must_use]
    pub fn proc0_attach(&mut self) -> PROC0_ATTACH_W<DBGFORCE_SPEC, 3> {
        PROC0_ATTACH_W::new(self)
    }
    #[doc = "Bit 5 - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    #[must_use]
    pub fn proc1_swdi(&mut self) -> PROC1_SWDI_W<DBGFORCE_SPEC, 5> {
        PROC1_SWDI_W::new(self)
    }
    #[doc = "Bit 6 - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    #[must_use]
    pub fn proc1_swclk(&mut self) -> PROC1_SWCLK_W<DBGFORCE_SPEC, 6> {
        PROC1_SWCLK_W::new(self)
    }
    #[doc = "Bit 7 - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    #[must_use]
    pub fn proc1_attach(&mut self) -> PROC1_ATTACH_W<DBGFORCE_SPEC, 7> {
        PROC1_ATTACH_W::new(self)
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
#[doc = "Directly control the SWD debug port of either processor  

You can [`read`](crate::generic::Reg::read) this register and get [`dbgforce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgforce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGFORCE_SPEC;
impl crate::RegisterSpec for DBGFORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgforce::R`](R) reader structure"]
impl crate::Readable for DBGFORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgforce::W`](W) writer structure"]
impl crate::Writable for DBGFORCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGFORCE to value 0x66"]
impl crate::Resettable for DBGFORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0x66;
}
