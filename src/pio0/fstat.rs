#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FSTAT_SPEC>;
#[doc = "Field `RXFULL` reader - State machine RX FIFO is full"]
pub type RXFULL_R = crate::FieldReader;
#[doc = "Field `RXEMPTY` reader - State machine RX FIFO is empty"]
pub type RXEMPTY_R = crate::FieldReader;
#[doc = "Field `TXFULL` reader - State machine TX FIFO is full"]
pub type TXFULL_R = crate::FieldReader;
#[doc = "Field `TXEMPTY` reader - State machine TX FIFO is empty"]
pub type TXEMPTY_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - State machine RX FIFO is full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - State machine RX FIFO is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - State machine TX FIFO is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - State machine TX FIFO is empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "FIFO status register  

You can [`read`](crate::Reg::read) this register and get [`fstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSTAT_SPEC;
impl crate::RegisterSpec for FSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FSTAT_SPEC {}
#[doc = "`reset()` method sets FSTAT to value 0x0f00_0f00"]
impl crate::Resettable for FSTAT_SPEC {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
