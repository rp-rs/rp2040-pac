#[doc = "Register `IC_COMP_PARAM_1` reader"]
pub type R = crate::R<IC_COMP_PARAM_1_SPEC>;
#[doc = "Field `APB_DATA_WIDTH` reader - APB data bus width is 32 bits"]
pub type APB_DATA_WIDTH_R = crate::FieldReader;
#[doc = "Field `MAX_SPEED_MODE` reader - MAX SPEED MODE = FAST MODE"]
pub type MAX_SPEED_MODE_R = crate::FieldReader;
#[doc = "Field `HC_COUNT_VALUES` reader - Programmable count values for each mode."]
pub type HC_COUNT_VALUES_R = crate::BitReader;
#[doc = "Field `INTR_IO` reader - COMBINED Interrupt outputs"]
pub type INTR_IO_R = crate::BitReader;
#[doc = "Field `HAS_DMA` reader - DMA handshaking signals are enabled"]
pub type HAS_DMA_R = crate::BitReader;
#[doc = "Field `ADD_ENCODED_PARAMS` reader - Encoded parameters not visible"]
pub type ADD_ENCODED_PARAMS_R = crate::BitReader;
#[doc = "Field `RX_BUFFER_DEPTH` reader - RX Buffer Depth = 16"]
pub type RX_BUFFER_DEPTH_R = crate::FieldReader;
#[doc = "Field `TX_BUFFER_DEPTH` reader - TX Buffer Depth = 16"]
pub type TX_BUFFER_DEPTH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - APB data bus width is 32 bits"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MAX SPEED MODE = FAST MODE"]
    #[inline(always)]
    pub fn max_speed_mode(&self) -> MAX_SPEED_MODE_R {
        MAX_SPEED_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Programmable count values for each mode."]
    #[inline(always)]
    pub fn hc_count_values(&self) -> HC_COUNT_VALUES_R {
        HC_COUNT_VALUES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMBINED Interrupt outputs"]
    #[inline(always)]
    pub fn intr_io(&self) -> INTR_IO_R {
        INTR_IO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA handshaking signals are enabled"]
    #[inline(always)]
    pub fn has_dma(&self) -> HAS_DMA_R {
        HAS_DMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Encoded parameters not visible"]
    #[inline(always)]
    pub fn add_encoded_params(&self) -> ADD_ENCODED_PARAMS_R {
        ADD_ENCODED_PARAMS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - RX Buffer Depth = 16"]
    #[inline(always)]
    pub fn rx_buffer_depth(&self) -> RX_BUFFER_DEPTH_R {
        RX_BUFFER_DEPTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX Buffer Depth = 16"]
    #[inline(always)]
    pub fn tx_buffer_depth(&self) -> TX_BUFFER_DEPTH_R {
        TX_BUFFER_DEPTH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters  

You can [`read`](crate::Reg::read) this register and get [`ic_comp_param_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_COMP_PARAM_1_SPEC;
impl crate::RegisterSpec for IC_COMP_PARAM_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_comp_param_1::R`](R) reader structure"]
impl crate::Readable for IC_COMP_PARAM_1_SPEC {}
#[doc = "`reset()` method sets IC_COMP_PARAM_1 to value 0"]
impl crate::Resettable for IC_COMP_PARAM_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
