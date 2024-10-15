#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `TXEIS` reader - Transmit FIFO empty interrupt status"]
pub type TXEIS_R = crate::BitReader;
#[doc = "Field `TXOIS` reader - Transmit FIFO overflow interrupt status"]
pub type TXOIS_R = crate::BitReader;
#[doc = "Field `RXUIS` reader - Receive FIFO underflow interrupt status"]
pub type RXUIS_R = crate::BitReader;
#[doc = "Field `RXOIS` reader - Receive FIFO overflow interrupt status"]
pub type RXOIS_R = crate::BitReader;
#[doc = "Field `RXFIS` reader - Receive FIFO full interrupt status"]
pub type RXFIS_R = crate::BitReader;
#[doc = "Field `MSTIS` reader - Multi-master contention interrupt status"]
pub type MSTIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt status"]
    #[inline(always)]
    pub fn txeis(&self) -> TXEIS_R {
        TXEIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn txois(&self) -> TXOIS_R {
        TXOIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt status"]
    #[inline(always)]
    pub fn rxuis(&self) -> RXUIS_R {
        RXUIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn rxois(&self) -> RXOIS_R {
        RXOIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt status"]
    #[inline(always)]
    pub fn rxfis(&self) -> RXFIS_R {
        RXFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-master contention interrupt status"]
    #[inline(always)]
    pub fn mstis(&self) -> MSTIS_R {
        MSTIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt status  

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
