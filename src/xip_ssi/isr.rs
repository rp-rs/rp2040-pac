#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `MSTIS`"]
pub type MSTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIS`"]
pub type RXFIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOIS`"]
pub type RXOIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUIS`"]
pub type RXUIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOIS`"]
pub type TXOIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEIS`"]
pub type TXEIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - Multi-master contention interrupt status"]
    #[inline(always)]
    pub fn mstis(&self) -> MSTIS_R {
        MSTIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt status"]
    #[inline(always)]
    pub fn rxfis(&self) -> RXFIS_R {
        RXFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn rxois(&self) -> RXOIS_R {
        RXOIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt status"]
    #[inline(always)]
    pub fn rxuis(&self) -> RXUIS_R {
        RXUIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn txois(&self) -> TXOIS_R {
        TXOIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty interrupt status"]
    #[inline(always)]
    pub fn txeis(&self) -> TXEIS_R {
        TXEIS_R::new((self.bits & 0x01) != 0)
    }
}
