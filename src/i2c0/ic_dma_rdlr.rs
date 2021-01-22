#[doc = "Reader of register IC_DMA_RDLR"]
pub type R = crate::R<u32, super::IC_DMA_RDLR>;
#[doc = "Writer for register IC_DMA_RDLR"]
pub type W = crate::W<u32, super::IC_DMA_RDLR>;
#[doc = "Register IC_DMA_RDLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IC_DMA_RDLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMARDL`"]
pub type DMARDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMARDL`"]
pub struct DMARDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DMARDL_W {
        DMARDL_W { w: self }
    }
}
