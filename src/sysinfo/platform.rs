#[doc = "Reader of register PLATFORM"]
pub type R = crate::R<u32, super::PLATFORM>;
#[doc = "Reader of field `ASIC`"]
pub type ASIC_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPGA`"]
pub type FPGA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn asic(&self) -> ASIC_R {
        ASIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fpga(&self) -> FPGA_R {
        FPGA_R::new((self.bits & 0x01) != 0)
    }
}
