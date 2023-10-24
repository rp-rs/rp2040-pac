#[doc = "Register `BUS_PRIORITY` reader"]
pub type R = crate::R<BUS_PRIORITY_SPEC>;
#[doc = "Register `BUS_PRIORITY` writer"]
pub type W = crate::W<BUS_PRIORITY_SPEC>;
#[doc = "Field `PROC0` reader - 0 - low priority, 1 - high priority"]
pub type PROC0_R = crate::BitReader;
#[doc = "Field `PROC0` writer - 0 - low priority, 1 - high priority"]
pub type PROC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROC1` reader - 0 - low priority, 1 - high priority"]
pub type PROC1_R = crate::BitReader;
#[doc = "Field `PROC1` writer - 0 - low priority, 1 - high priority"]
pub type PROC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_R` reader - 0 - low priority, 1 - high priority"]
pub type DMA_R_R = crate::BitReader;
#[doc = "Field `DMA_R` writer - 0 - low priority, 1 - high priority"]
pub type DMA_R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_W` reader - 0 - low priority, 1 - high priority"]
pub type DMA_W_R = crate::BitReader;
#[doc = "Field `DMA_W` writer - 0 - low priority, 1 - high priority"]
pub type DMA_W_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_r(&self) -> DMA_R_R {
        DMA_R_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_w(&self) -> DMA_W_R {
        DMA_W_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn proc0(&mut self) -> PROC0_W<BUS_PRIORITY_SPEC, 0> {
        PROC0_W::new(self)
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<BUS_PRIORITY_SPEC, 4> {
        PROC1_W::new(self)
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn dma_r(&mut self) -> DMA_R_W<BUS_PRIORITY_SPEC, 8> {
        DMA_R_W::new(self)
    }
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn dma_w(&mut self) -> DMA_W_W<BUS_PRIORITY_SPEC, 12> {
        DMA_W_W::new(self)
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
#[doc = "Set the priority of each master for bus arbitration.  

You can [`read`](crate::generic::Reg::read) this register and get [`bus_priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_PRIORITY_SPEC;
impl crate::RegisterSpec for BUS_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_priority::R`](R) reader structure"]
impl crate::Readable for BUS_PRIORITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_priority::W`](W) writer structure"]
impl crate::Writable for BUS_PRIORITY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_PRIORITY to value 0"]
impl crate::Resettable for BUS_PRIORITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
