#[doc = "Register `INTERP1_CTRL_LANE1` reader"]
pub struct R(crate::R<INTERP1_CTRL_LANE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP1_CTRL_LANE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP1_CTRL_LANE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP1_CTRL_LANE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP1_CTRL_LANE1` writer"]
pub struct W(crate::W<INTERP1_CTRL_LANE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP1_CTRL_LANE1_SPEC>;
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
impl From<crate::W<INTERP1_CTRL_LANE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERP1_CTRL_LANE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_MSB` reader - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
pub struct FORCE_MSB_R(crate::FieldReader<u8, u8>);
impl FORCE_MSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORCE_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_MSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_MSB` writer - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
pub struct FORCE_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "Field `ADD_RAW` reader - If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
pub struct ADD_RAW_R(crate::FieldReader<bool, bool>);
impl ADD_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADD_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD_RAW` writer - If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CROSS_RESULT` reader - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
pub struct CROSS_RESULT_R(crate::FieldReader<bool, bool>);
impl CROSS_RESULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CROSS_RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CROSS_RESULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CROSS_RESULT` writer - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CROSS_INPUT` reader - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
pub struct CROSS_INPUT_R(crate::FieldReader<bool, bool>);
impl CROSS_INPUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CROSS_INPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CROSS_INPUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CROSS_INPUT` writer - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SIGNED` reader - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
pub struct SIGNED_R(crate::FieldReader<bool, bool>);
impl SIGNED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIGNED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGNED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGNED` writer - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MASK_MSB` reader - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
pub struct MASK_MSB_R(crate::FieldReader<u8, u8>);
impl MASK_MSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASK_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_MSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_MSB` writer - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
pub struct MASK_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `MASK_LSB` reader - The least-significant bit allowed to pass by the mask (inclusive)"]
pub struct MASK_LSB_R(crate::FieldReader<u8, u8>);
impl MASK_LSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASK_LSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_LSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_LSB` writer - The least-significant bit allowed to pass by the mask (inclusive)"]
pub struct MASK_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_LSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `SHIFT` reader - Logical right-shift applied to accumulator before masking"]
pub struct SHIFT_R(crate::FieldReader<u8, u8>);
impl SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` writer - Logical right-shift applied to accumulator before masking"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:20 - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
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
    #[doc = "Bit 16 - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn cross_input(&self) -> CROSS_INPUT_R {
        CROSS_INPUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn signed(&self) -> SIGNED_R {
        SIGNED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 10:14 - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
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
    #[doc = "Bits 19:20 - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
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
    #[doc = "Bit 16 - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn cross_input(&mut self) -> CROSS_INPUT_W {
        CROSS_INPUT_W { w: self }
    }
    #[doc = "Bit 15 - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn signed(&mut self) -> SIGNED_W {
        SIGNED_W { w: self }
    }
    #[doc = "Bits 10:14 - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for lane 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp1_ctrl_lane1](index.html) module"]
pub struct INTERP1_CTRL_LANE1_SPEC;
impl crate::RegisterSpec for INTERP1_CTRL_LANE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp1_ctrl_lane1::R](R) reader structure"]
impl crate::Readable for INTERP1_CTRL_LANE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp1_ctrl_lane1::W](W) writer structure"]
impl crate::Writable for INTERP1_CTRL_LANE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERP1_CTRL_LANE1 to value 0"]
impl crate::Resettable for INTERP1_CTRL_LANE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
