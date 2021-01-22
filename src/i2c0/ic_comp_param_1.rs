#[doc = "Reader of register IC_COMP_PARAM_1"]
pub type R = crate::R<u32, super::IC_COMP_PARAM_1>;
#[doc = "Reader of field `TX_BUFFER_DEPTH`"]
pub type TX_BUFFER_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX_BUFFER_DEPTH`"]
pub type RX_BUFFER_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADD_ENCODED_PARAMS`"]
pub type ADD_ENCODED_PARAMS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HAS_DMA`"]
pub type HAS_DMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR_IO`"]
pub type INTR_IO_R = crate::R<bool, bool>;
#[doc = "Reader of field `HC_COUNT_VALUES`"]
pub type HC_COUNT_VALUES_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAX_SPEED_MODE`"]
pub type MAX_SPEED_MODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `APB_DATA_WIDTH`"]
pub type APB_DATA_WIDTH_R = crate::R<u8, u8>;
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
