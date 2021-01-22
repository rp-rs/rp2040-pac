#[doc = "Reader of register DBG_CFGINFO"]
pub type R = crate::R<u32, super::DBG_CFGINFO>;
#[doc = "Reader of field `IMEM_SIZE`"]
pub type IMEM_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SM_COUNT`"]
pub type SM_COUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFO_DEPTH`"]
pub type FIFO_DEPTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:21 - The size of the instruction memory, measured in units of one instruction"]
    #[inline(always)]
    pub fn imem_size(&self) -> IMEM_SIZE_R {
        IMEM_SIZE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - The number of state machines this PIO instance is equipped with."]
    #[inline(always)]
    pub fn sm_count(&self) -> SM_COUNT_R {
        SM_COUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5 - The depth of the state machine TX/RX FIFOs, measured in words.\\n Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double\\n this depth."]
    #[inline(always)]
    pub fn fifo_depth(&self) -> FIFO_DEPTH_R {
        FIFO_DEPTH_R::new((self.bits & 0x3f) as u8)
    }
}
