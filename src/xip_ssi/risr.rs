#[doc = "Reader of register RISR"]
pub type R = crate::R<u32, super::RISR>;
#[doc = "Reader of field `MSTIR`"]
pub type MSTIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIR`"]
pub type RXFIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOIR`"]
pub type RXOIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUIR`"]
pub type RXUIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOIR`"]
pub type TXOIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEIR`"]
pub type TXEIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - Multi-master contention raw interrupt status"]
    #[inline(always)]
    pub fn mstir(&self) -> MSTIR_R {
        MSTIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxfir(&self) -> RXFIR_R {
        RXFIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn rxoir(&self) -> RXOIR_R {
        RXOIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow raw interrupt status"]
    #[inline(always)]
    pub fn rxuir(&self) -> RXUIR_R {
        RXUIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn txoir(&self) -> TXOIR_R {
        TXOIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txeir(&self) -> TXEIR_R {
        TXEIR_R::new((self.bits & 0x01) != 0)
    }
}
