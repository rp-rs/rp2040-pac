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
pub type DMARDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
pub type DMARDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IC_DMA_RDLR_SPEC, u8, u8, 4, O>;
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
    #[must_use]
    pub fn dmardl(&mut self) -> DMARDL_W<0> {
        DMARDL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_DMA_RDLR to value 0"]
impl crate::Resettable for IC_DMA_RDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
