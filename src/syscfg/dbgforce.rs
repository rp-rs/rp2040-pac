#[doc = "Reader of register DBGFORCE"]
pub type R = crate::R<u32, super::DBGFORCE>;
#[doc = "Writer for register DBGFORCE"]
pub type W = crate::W<u32, super::DBGFORCE>;
#[doc = "Register DBGFORCE `reset()`'s with value 0x66"]
impl crate::ResetValue for super::DBGFORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x66
    }
}
#[doc = "Reader of field `PROC1_ATTACH`"]
pub type PROC1_ATTACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC1_ATTACH`"]
pub struct PROC1_ATTACH_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_ATTACH_W<'a> {
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
#[doc = "Reader of field `PROC1_SWCLK`"]
pub type PROC1_SWCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC1_SWCLK`"]
pub struct PROC1_SWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_SWCLK_W<'a> {
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
#[doc = "Reader of field `PROC1_SWDI`"]
pub type PROC1_SWDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC1_SWDI`"]
pub struct PROC1_SWDI_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_SWDI_W<'a> {
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
#[doc = "Reader of field `PROC1_SWDO`"]
pub type PROC1_SWDO_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROC0_ATTACH`"]
pub type PROC0_ATTACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC0_ATTACH`"]
pub struct PROC0_ATTACH_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_ATTACH_W<'a> {
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
#[doc = "Reader of field `PROC0_SWCLK`"]
pub type PROC0_SWCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC0_SWCLK`"]
pub struct PROC0_SWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_SWCLK_W<'a> {
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
#[doc = "Reader of field `PROC0_SWDI`"]
pub type PROC0_SWDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC0_SWDI`"]
pub struct PROC0_SWDI_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_SWDI_W<'a> {
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
#[doc = "Reader of field `PROC0_SWDO`"]
pub type PROC0_SWDO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc1_attach(&self) -> PROC1_ATTACH_R {
        PROC1_ATTACH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swclk(&self) -> PROC1_SWCLK_R {
        PROC1_SWCLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swdi(&self) -> PROC1_SWDI_R {
        PROC1_SWDI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Observe the value of processor 1 SWDIO output."]
    #[inline(always)]
    pub fn proc1_swdo(&self) -> PROC1_SWDO_R {
        PROC1_SWDO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc0_attach(&self) -> PROC0_ATTACH_R {
        PROC0_ATTACH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swclk(&self) -> PROC0_SWCLK_R {
        PROC0_SWCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swdi(&self) -> PROC0_SWDI_R {
        PROC0_SWDI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Observe the value of processor 0 SWDIO output."]
    #[inline(always)]
    pub fn proc0_swdo(&self) -> PROC0_SWDO_R {
        PROC0_SWDO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc1_attach(&mut self) -> PROC1_ATTACH_W {
        PROC1_ATTACH_W { w: self }
    }
    #[doc = "Bit 6 - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swclk(&mut self) -> PROC1_SWCLK_W {
        PROC1_SWCLK_W { w: self }
    }
    #[doc = "Bit 5 - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swdi(&mut self) -> PROC1_SWDI_W {
        PROC1_SWDI_W { w: self }
    }
    #[doc = "Bit 3 - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc0_attach(&mut self) -> PROC0_ATTACH_W {
        PROC0_ATTACH_W { w: self }
    }
    #[doc = "Bit 2 - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swclk(&mut self) -> PROC0_SWCLK_W {
        PROC0_SWCLK_W { w: self }
    }
    #[doc = "Bit 1 - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swdi(&mut self) -> PROC0_SWDI_W {
        PROC0_SWDI_W { w: self }
    }
}
