#[doc = "Reader of register FSTAT"]
pub type R = crate::R<u32, super::FSTAT>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFULL`"]
pub type TXFULL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXEMPTY`"]
pub type RXEMPTY_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - State machine TX FIFO is empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - State machine TX FIFO is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - State machine RX FIFO is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - State machine RX FIFO is full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new((self.bits & 0x0f) as u8)
    }
}
