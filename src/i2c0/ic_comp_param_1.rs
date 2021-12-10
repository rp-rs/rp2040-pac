#[doc = "Register `IC_COMP_PARAM_1` reader"]
pub struct R(crate::R<IC_COMP_PARAM_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_COMP_PARAM_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_COMP_PARAM_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_COMP_PARAM_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_BUFFER_DEPTH` reader - TX Buffer Depth = 16"]
pub struct TX_BUFFER_DEPTH_R(crate::FieldReader<u8, u8>);
impl TX_BUFFER_DEPTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BUFFER_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BUFFER_DEPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BUFFER_DEPTH` reader - RX Buffer Depth = 16"]
pub struct RX_BUFFER_DEPTH_R(crate::FieldReader<u8, u8>);
impl RX_BUFFER_DEPTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_BUFFER_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BUFFER_DEPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD_ENCODED_PARAMS` reader - Encoded parameters not visible"]
pub struct ADD_ENCODED_PARAMS_R(crate::FieldReader<bool, bool>);
impl ADD_ENCODED_PARAMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADD_ENCODED_PARAMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_ENCODED_PARAMS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HAS_DMA` reader - DMA handshaking signals are enabled"]
pub struct HAS_DMA_R(crate::FieldReader<bool, bool>);
impl HAS_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HAS_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HAS_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTR_IO` reader - COMBINED Interrupt outputs"]
pub struct INTR_IO_R(crate::FieldReader<bool, bool>);
impl INTR_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_IO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HC_COUNT_VALUES` reader - Programmable count values for each mode."]
pub struct HC_COUNT_VALUES_R(crate::FieldReader<bool, bool>);
impl HC_COUNT_VALUES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HC_COUNT_VALUES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HC_COUNT_VALUES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_SPEED_MODE` reader - MAX SPEED MODE = FAST MODE"]
pub struct MAX_SPEED_MODE_R(crate::FieldReader<u8, u8>);
impl MAX_SPEED_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAX_SPEED_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_SPEED_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_DATA_WIDTH` reader - APB data bus width is 32 bits"]
pub struct APB_DATA_WIDTH_R(crate::FieldReader<u8, u8>);
impl APB_DATA_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APB_DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_DATA_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:23 - TX Buffer Depth = 16"]
    #[inline(always)]
    pub fn tx_buffer_depth(&self) -> TX_BUFFER_DEPTH_R {
        TX_BUFFER_DEPTH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Buffer Depth = 16"]
    #[inline(always)]
    pub fn rx_buffer_depth(&self) -> RX_BUFFER_DEPTH_R {
        RX_BUFFER_DEPTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - Encoded parameters not visible"]
    #[inline(always)]
    pub fn add_encoded_params(&self) -> ADD_ENCODED_PARAMS_R {
        ADD_ENCODED_PARAMS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA handshaking signals are enabled"]
    #[inline(always)]
    pub fn has_dma(&self) -> HAS_DMA_R {
        HAS_DMA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COMBINED Interrupt outputs"]
    #[inline(always)]
    pub fn intr_io(&self) -> INTR_IO_R {
        INTR_IO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Programmable count values for each mode."]
    #[inline(always)]
    pub fn hc_count_values(&self) -> HC_COUNT_VALUES_R {
        HC_COUNT_VALUES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - MAX SPEED MODE = FAST MODE"]
    #[inline(always)]
    pub fn max_speed_mode(&self) -> MAX_SPEED_MODE_R {
        MAX_SPEED_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - APB data bus width is 32 bits"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_comp_param_1](index.html) module"]
pub struct IC_COMP_PARAM_1_SPEC;
impl crate::RegisterSpec for IC_COMP_PARAM_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_comp_param_1::R](R) reader structure"]
impl crate::Readable for IC_COMP_PARAM_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_COMP_PARAM_1 to value 0"]
impl crate::Resettable for IC_COMP_PARAM_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
