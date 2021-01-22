#[doc = "Reader of register FRCE_OFF"]
pub type R = crate::R<u32, super::FRCE_OFF>;
#[doc = "Writer for register FRCE_OFF"]
pub type W = crate::W<u32, super::FRCE_OFF>;
#[doc = "Register FRCE_OFF `reset()`'s with value 0"]
impl crate::ResetValue for super::FRCE_OFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `proc1`"]
pub type PROC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `proc1`"]
pub struct PROC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `proc0`"]
pub type PROC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `proc0`"]
pub struct PROC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `sio`"]
pub type SIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sio`"]
pub struct SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `vreg_and_chip_reset`"]
pub type VREG_AND_CHIP_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `vreg_and_chip_reset`"]
pub struct VREG_AND_CHIP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_AND_CHIP_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `xip`"]
pub type XIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xip`"]
pub struct XIP_W<'a> {
    w: &'a mut W,
}
impl<'a> XIP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `sram5`"]
pub type SRAM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram5`"]
pub struct SRAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `sram4`"]
pub type SRAM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram4`"]
pub struct SRAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `sram3`"]
pub type SRAM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram3`"]
pub struct SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `sram2`"]
pub type SRAM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram2`"]
pub struct SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `sram1`"]
pub type SRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram1`"]
pub struct SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `sram0`"]
pub type SRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram0`"]
pub struct SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `rom`"]
pub type ROM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rom`"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `busfabric`"]
pub type BUSFABRIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `busfabric`"]
pub struct BUSFABRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFABRIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `resets`"]
pub type RESETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `resets`"]
pub struct RESETS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `clocks`"]
pub type CLOCKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clocks`"]
pub struct CLOCKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `xosc`"]
pub type XOSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xosc`"]
pub struct XOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `rosc`"]
pub type ROSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rosc`"]
pub struct ROSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
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
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&mut self) -> PROC1_W {
        PROC1_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&mut self) -> PROC0_W {
        PROC0_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&mut self) -> SIO_W {
        SIO_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&mut self) -> VREG_AND_CHIP_RESET_W {
        VREG_AND_CHIP_RESET_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&mut self) -> XIP_W {
        XIP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&mut self) -> SRAM5_W {
        SRAM5_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&mut self) -> SRAM4_W {
        SRAM4_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W {
        SRAM3_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W {
        SRAM2_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W {
        SRAM1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&mut self) -> SRAM0_W {
        SRAM0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&mut self) -> BUSFABRIC_W {
        BUSFABRIC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&mut self) -> RESETS_W {
        RESETS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&mut self) -> CLOCKS_W {
        CLOCKS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&mut self) -> XOSC_W {
        XOSC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&mut self) -> ROSC_W {
        ROSC_W { w: self }
    }
}
