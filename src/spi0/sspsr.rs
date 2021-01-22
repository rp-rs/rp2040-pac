#[doc = "Reader of register SSPSR"]
pub type R = crate::R<u32, super::SSPSR>;
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RNE`"]
pub type RNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TNF`"]
pub type TNF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
}
