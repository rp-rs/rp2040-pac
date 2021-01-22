#[doc = "Reader of register DONE"]
pub type R = crate::R<u32, super::DONE>;
#[doc = "Reader of field `proc1`"]
pub type PROC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `proc0`"]
pub type PROC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `sio`"]
pub type SIO_R = crate::R<bool, bool>;
#[doc = "Reader of field `vreg_and_chip_reset`"]
pub type VREG_AND_CHIP_RESET_R = crate::R<bool, bool>;
#[doc = "Reader of field `xip`"]
pub type XIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `sram5`"]
pub type SRAM5_R = crate::R<bool, bool>;
#[doc = "Reader of field `sram4`"]
pub type SRAM4_R = crate::R<bool, bool>;
#[doc = "Reader of field `sram3`"]
pub type SRAM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `sram2`"]
pub type SRAM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `sram1`"]
pub type SRAM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `sram0`"]
pub type SRAM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `rom`"]
pub type ROM_R = crate::R<bool, bool>;
#[doc = "Reader of field `busfabric`"]
pub type BUSFABRIC_R = crate::R<bool, bool>;
#[doc = "Reader of field `resets`"]
pub type RESETS_R = crate::R<bool, bool>;
#[doc = "Reader of field `clocks`"]
pub type CLOCKS_R = crate::R<bool, bool>;
#[doc = "Reader of field `xosc`"]
pub type XOSC_R = crate::R<bool, bool>;
#[doc = "Reader of field `rosc`"]
pub type ROSC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&self) -> VREG_AND_CHIP_RESET_R {
        VREG_AND_CHIP_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&self) -> BUSFABRIC_R {
        BUSFABRIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&self) -> RESETS_R {
        RESETS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&self) -> CLOCKS_R {
        CLOCKS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&self) -> ROSC_R {
        ROSC_R::new((self.bits & 0x01) != 0)
    }
}
