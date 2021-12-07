#[doc = "Register `IC_DMA_RDLR` reader"]
pub struct R(crate::R<IC_DMA_RDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_DMA_RDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_DMA_RDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_DMA_RDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_DMA_RDLR` writer"]
pub struct W(crate::W<IC_DMA_RDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_DMA_RDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IC_DMA_RDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_DMA_RDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARDL` reader - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
pub struct DMARDL_R(crate::FieldReader<u8, u8>);
impl DMARDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMARDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMARDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
pub struct DMARDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DMARDL_W {
        DMARDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Receive Data Level Register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_dma_rdlr](index.html) module"]
pub struct IC_DMA_RDLR_SPEC;
impl crate::RegisterSpec for IC_DMA_RDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_dma_rdlr::R](R) reader structure"]
impl crate::Readable for IC_DMA_RDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_dma_rdlr::W](W) writer structure"]
impl crate::Writable for IC_DMA_RDLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_DMA_RDLR to value 0"]
impl crate::Resettable for IC_DMA_RDLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
