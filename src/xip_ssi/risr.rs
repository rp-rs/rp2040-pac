#[doc = "Register `RISR` reader"]
pub type R = crate::R<RISR_SPEC>;
#[doc = "Register `RISR` writer"]
pub type W = crate::W<RISR_SPEC>;
#[doc = "Field `TXEIR` reader - Transmit FIFO empty raw interrupt status"]
pub type TXEIR_R = crate::BitReader;
#[doc = "Field `TXOIR` reader - Transmit FIFO overflow raw interrupt status"]
pub type TXOIR_R = crate::BitReader;
#[doc = "Field `RXUIR` reader - Receive FIFO underflow raw interrupt status"]
pub type RXUIR_R = crate::BitReader;
#[doc = "Field `RXOIR` reader - Receive FIFO overflow raw interrupt status"]
pub type RXOIR_R = crate::BitReader;
#[doc = "Field `RXFIR` reader - Receive FIFO full raw interrupt status"]
pub type RXFIR_R = crate::BitReader;
#[doc = "Field `MSTIR` reader - Multi-master contention raw interrupt status"]
pub type MSTIR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txeir(&self) -> TXEIR_R {
        TXEIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn txoir(&self) -> TXOIR_R {
        TXOIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow raw interrupt status"]
    #[inline(always)]
    pub fn rxuir(&self) -> RXUIR_R {
        RXUIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn rxoir(&self) -> RXOIR_R {
        RXOIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxfir(&self) -> RXFIR_R {
        RXFIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-master contention raw interrupt status"]
    #[inline(always)]
    pub fn mstir(&self) -> MSTIR_R {
        MSTIR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {}
#[doc = "Raw interrupt status  

You can [`read`](crate::generic::Reg::read) this register and get [`risr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`risr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISR_SPEC;
impl crate::RegisterSpec for RISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`risr::R`](R) reader structure"]
impl crate::Readable for RISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`risr::W`](W) writer structure"]
impl crate::Writable for RISR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RISR to value 0"]
impl crate::Resettable for RISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
