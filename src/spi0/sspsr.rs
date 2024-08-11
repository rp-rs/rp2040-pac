#[doc = "Register `SSPSR` reader"]
pub type R = crate::R<SSPSR_SPEC>;
#[doc = "Register `SSPSR` writer"]
pub type W = crate::W<SSPSR_SPEC>;
#[doc = "Field `TFE` reader - Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TNF` reader - Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
pub type TNF_R = crate::BitReader;
#[doc = "Field `RNE` reader - Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
pub type RNE_R = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
pub type RFF_R = crate::BitReader;
#[doc = "Field `BSY` reader - PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {}
#[doc = "Status register, SSPSR on page 3-7  

You can [`read`](crate::generic::Reg::read) this register and get [`sspsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPSR_SPEC;
impl crate::RegisterSpec for SSPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspsr::R`](R) reader structure"]
impl crate::Readable for SSPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspsr::W`](W) writer structure"]
impl crate::Writable for SSPSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPSR to value 0x03"]
impl crate::Resettable for SSPSR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
