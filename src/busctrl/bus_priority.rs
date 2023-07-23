#[doc = "Register `BUS_PRIORITY` reader"]
pub struct R(crate::R<BUS_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_PRIORITY` writer"]
pub struct W(crate::W<BUS_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_PRIORITY_SPEC>;
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
impl From<crate::W<BUS_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROC0` reader - 0 - low priority, 1 - high priority"]
pub type PROC0_R = crate::BitReader<bool>;
#[doc = "Field `PROC0` writer - 0 - low priority, 1 - high priority"]
pub type PROC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_PRIORITY_SPEC, bool, O>;
#[doc = "Field `PROC1` reader - 0 - low priority, 1 - high priority"]
pub type PROC1_R = crate::BitReader<bool>;
#[doc = "Field `PROC1` writer - 0 - low priority, 1 - high priority"]
pub type PROC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_PRIORITY_SPEC, bool, O>;
#[doc = "Field `DMA_R` reader - 0 - low priority, 1 - high priority"]
pub type DMA_R_R = crate::BitReader<bool>;
#[doc = "Field `DMA_R` writer - 0 - low priority, 1 - high priority"]
pub type DMA_R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_PRIORITY_SPEC, bool, O>;
#[doc = "Field `DMA_W` reader - 0 - low priority, 1 - high priority"]
pub type DMA_W_R = crate::BitReader<bool>;
#[doc = "Field `DMA_W` writer - 0 - low priority, 1 - high priority"]
pub type DMA_W_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_PRIORITY_SPEC, bool, O>;
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
    pub fn proc0(&mut self) -> PROC0_W<0> {
        PROC0_W::new(self)
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<4> {
        PROC1_W::new(self)
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn dma_r(&mut self) -> DMA_R_W<8> {
        DMA_R_W::new(self)
    }
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    #[must_use]
    pub fn dma_w(&mut self) -> DMA_W_W<12> {
        DMA_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the priority of each master for bus arbitration.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [bus_priority](index.html) module"]
pub struct BUS_PRIORITY_SPEC;
impl crate::RegisterSpec for BUS_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_priority::R](R) reader structure"]
impl crate::Readable for BUS_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_priority::W](W) writer structure"]
impl crate::Writable for BUS_PRIORITY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_PRIORITY to value 0"]
impl crate::Resettable for BUS_PRIORITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
