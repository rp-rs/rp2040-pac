#[doc = "Reader of register INTERP0_CTRL_LANE1"]
pub type R = crate::R<u32, super::INTERP0_CTRL_LANE1>;
#[doc = "Writer for register INTERP0_CTRL_LANE1"]
pub type W = crate::W<u32, super::INTERP0_CTRL_LANE1>;
#[doc = "Register INTERP0_CTRL_LANE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERP0_CTRL_LANE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCE_MSB`"]
pub type FORCE_MSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORCE_MSB`"]
pub struct FORCE_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `ADD_RAW`"]
pub type ADD_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD_RAW`"]
pub struct ADD_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CROSS_RESULT`"]
pub type CROSS_RESULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CROSS_RESULT`"]
pub struct CROSS_RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> CROSS_RESULT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CROSS_INPUT`"]
pub type CROSS_INPUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CROSS_INPUT`"]
pub struct CROSS_INPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CROSS_INPUT_W<'a> {
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
#[doc = "Reader of field `SIGNED`"]
pub type SIGNED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGNED`"]
pub struct SIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNED_W<'a> {
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
#[doc = "Reader of field `MASK_MSB`"]
pub type MASK_MSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK_MSB`"]
pub struct MASK_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `MASK_LSB`"]
pub type MASK_LSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK_LSB`"]
pub struct MASK_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_LSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SHIFT`"]
pub type SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHIFT`"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:20 - ORed into bits 29:28 of the lane result presented to the processor on the bus.\\n No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence\\n of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn force_msb(&self) -> FORCE_MSB_R {
        FORCE_MSB_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn add_raw(&self) -> ADD_RAW_R {
        ADD_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn cross_result(&self) -> CROSS_RESULT_R {
        CROSS_RESULT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.\\n Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn cross_input(&self) -> CROSS_INPUT_R {
        CROSS_INPUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits\\n before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn signed(&self) -> SIGNED_R {
        SIGNED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 10:14 - The most-significant bit allowed to pass by the mask (inclusive)\\n Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn mask_msb(&self) -> MASK_MSB_R {
        MASK_MSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn mask_lsb(&self) -> MASK_LSB_R {
        MASK_LSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Logical right-shift applied to accumulator before masking"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 19:20 - ORed into bits 29:28 of the lane result presented to the processor on the bus.\\n No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence\\n of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn force_msb(&mut self) -> FORCE_MSB_W {
        FORCE_MSB_W { w: self }
    }
    #[doc = "Bit 18 - If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn add_raw(&mut self) -> ADD_RAW_W {
        ADD_RAW_W { w: self }
    }
    #[doc = "Bit 17 - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn cross_result(&mut self) -> CROSS_RESULT_W {
        CROSS_RESULT_W { w: self }
    }
    #[doc = "Bit 16 - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.\\n Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn cross_input(&mut self) -> CROSS_INPUT_W {
        CROSS_INPUT_W { w: self }
    }
    #[doc = "Bit 15 - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits\\n before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn signed(&mut self) -> SIGNED_W {
        SIGNED_W { w: self }
    }
    #[doc = "Bits 10:14 - The most-significant bit allowed to pass by the mask (inclusive)\\n Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn mask_msb(&mut self) -> MASK_MSB_W {
        MASK_MSB_W { w: self }
    }
    #[doc = "Bits 5:9 - The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn mask_lsb(&mut self) -> MASK_LSB_W {
        MASK_LSB_W { w: self }
    }
    #[doc = "Bits 0:4 - Logical right-shift applied to accumulator before masking"]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
}
