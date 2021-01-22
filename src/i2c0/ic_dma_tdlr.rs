#[doc = "Reader of register IC_DMA_TDLR"]
pub type R = crate::R<u32, super::IC_DMA_TDLR>;
#[doc = "Writer for register IC_DMA_TDLR"]
pub type W = crate::W<u32, super::IC_DMA_TDLR>;
#[doc = "Register IC_DMA_TDLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IC_DMA_TDLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATDL`"]
pub type DMATDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMATDL`"]
pub struct DMATDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn dmatdl(&self) -> DMATDL_R {
        DMATDL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn dmatdl(&mut self) -> DMATDL_W {
        DMATDL_W { w: self }
    }
}
