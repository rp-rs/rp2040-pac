#[doc = "Register `INTERP0_CTRL_LANE0` reader"]
pub struct R(crate::R<INTERP0_CTRL_LANE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP0_CTRL_LANE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP0_CTRL_LANE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP0_CTRL_LANE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP0_CTRL_LANE0` writer"]
pub struct W(crate::W<INTERP0_CTRL_LANE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP0_CTRL_LANE0_SPEC>;
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
impl From<crate::W<INTERP0_CTRL_LANE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERP0_CTRL_LANE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFT` reader - Logical right-shift applied to accumulator before masking"]
pub type SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHIFT` writer - Logical right-shift applied to accumulator before masking"]
pub type SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MASK_LSB` reader - The least-significant bit allowed to pass by the mask (inclusive)"]
pub type MASK_LSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_LSB` writer - The least-significant bit allowed to pass by the mask (inclusive)"]
pub type MASK_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MASK_MSB` reader - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
pub type MASK_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_MSB` writer - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
pub type MASK_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, u8, u8, 5, O>;
#[doc = "Field `SIGNED` reader - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
pub type SIGNED_R = crate::BitReader<bool>;
#[doc = "Field `SIGNED` writer - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
pub type SIGNED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, bool, O>;
#[doc = "Field `CROSS_INPUT` reader - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
pub type CROSS_INPUT_R = crate::BitReader<bool>;
#[doc = "Field `CROSS_INPUT` writer - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
pub type CROSS_INPUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, bool, O>;
#[doc = "Field `CROSS_RESULT` reader - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
pub type CROSS_RESULT_R = crate::BitReader<bool>;
#[doc = "Field `CROSS_RESULT` writer - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
pub type CROSS_RESULT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, bool, O>;
#[doc = "Field `ADD_RAW` reader - If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
pub type ADD_RAW_R = crate::BitReader<bool>;
#[doc = "Field `ADD_RAW` writer - If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
pub type ADD_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, bool, O>;
#[doc = "Field `FORCE_MSB` reader - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
pub type FORCE_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORCE_MSB` writer - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
pub type FORCE_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLEND` reader - Only present on INTERP0 on each core. If BLEND mode is enabled:  
 - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled  
 by the 8 LSBs of lane 1 shift and mask value (a fractional number between  
 0 and 255/256ths)  
 - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)  
 - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)  
 LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
pub type BLEND_R = crate::BitReader<bool>;
#[doc = "Field `BLEND` writer - Only present on INTERP0 on each core. If BLEND mode is enabled:  
 - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled  
 by the 8 LSBs of lane 1 shift and mask value (a fractional number between  
 0 and 255/256ths)  
 - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)  
 - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)  
 LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
pub type BLEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERP0_CTRL_LANE0_SPEC, bool, O>;
#[doc = "Field `OVERF0` reader - Indicates if any masked-off MSBs in ACCUM0 are set."]
pub type OVERF0_R = crate::BitReader<bool>;
#[doc = "Field `OVERF1` reader - Indicates if any masked-off MSBs in ACCUM1 are set."]
pub type OVERF1_R = crate::BitReader<bool>;
#[doc = "Field `OVERF` reader - Set if either OVERF0 or OVERF1 is set."]
pub type OVERF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - Logical right-shift applied to accumulator before masking"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn mask_lsb(&self) -> MASK_LSB_R {
        MASK_LSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn mask_msb(&self) -> MASK_MSB_R {
        MASK_MSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn signed(&self) -> SIGNED_R {
        SIGNED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn cross_input(&self) -> CROSS_INPUT_R {
        CROSS_INPUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn cross_result(&self) -> CROSS_RESULT_R {
        CROSS_RESULT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn add_raw(&self) -> ADD_RAW_R {
        ADD_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn force_msb(&self) -> FORCE_MSB_R {
        FORCE_MSB_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - Only present on INTERP0 on each core. If BLEND mode is enabled:  
 - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled  
 by the 8 LSBs of lane 1 shift and mask value (a fractional number between  
 0 and 255/256ths)  
 - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)  
 - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)  
 LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
    #[inline(always)]
    pub fn blend(&self) -> BLEND_R {
        BLEND_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicates if any masked-off MSBs in ACCUM0 are set."]
    #[inline(always)]
    pub fn overf0(&self) -> OVERF0_R {
        OVERF0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates if any masked-off MSBs in ACCUM1 are set."]
    #[inline(always)]
    pub fn overf1(&self) -> OVERF1_R {
        OVERF1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set if either OVERF0 or OVERF1 is set."]
    #[inline(always)]
    pub fn overf(&self) -> OVERF_R {
        OVERF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Logical right-shift applied to accumulator before masking"]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> SHIFT_W<0> {
        SHIFT_W::new(self)
    }
    #[doc = "Bits 5:9 - The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    #[must_use]
    pub fn mask_lsb(&mut self) -> MASK_LSB_W<5> {
        MASK_LSB_W::new(self)
    }
    #[doc = "Bits 10:14 - The most-significant bit allowed to pass by the mask (inclusive)  
 Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    #[must_use]
    pub fn mask_msb(&mut self) -> MASK_MSB_W<10> {
        MASK_MSB_W::new(self)
    }
    #[doc = "Bit 15 - If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits  
 before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    #[must_use]
    pub fn signed(&mut self) -> SIGNED_W<15> {
        SIGNED_W::new(self)
    }
    #[doc = "Bit 16 - If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.  
 Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    #[must_use]
    pub fn cross_input(&mut self) -> CROSS_INPUT_W<16> {
        CROSS_INPUT_W::new(self)
    }
    #[doc = "Bit 17 - If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    #[must_use]
    pub fn cross_result(&mut self) -> CROSS_RESULT_W<17> {
        CROSS_RESULT_W::new(self)
    }
    #[doc = "Bit 18 - If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    #[inline(always)]
    #[must_use]
    pub fn add_raw(&mut self) -> ADD_RAW_W<18> {
        ADD_RAW_W::new(self)
    }
    #[doc = "Bits 19:20 - ORed into bits 29:28 of the lane result presented to the processor on the bus.  
 No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence  
 of pointers into flash or SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn force_msb(&mut self) -> FORCE_MSB_W<19> {
        FORCE_MSB_W::new(self)
    }
    #[doc = "Bit 21 - Only present on INTERP0 on each core. If BLEND mode is enabled:  
 - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled  
 by the 8 LSBs of lane 1 shift and mask value (a fractional number between  
 0 and 255/256ths)  
 - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)  
 - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)  
 LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
    #[inline(always)]
    #[must_use]
    pub fn blend(&mut self) -> BLEND_W<21> {
        BLEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for lane 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp0_ctrl_lane0](index.html) module"]
pub struct INTERP0_CTRL_LANE0_SPEC;
impl crate::RegisterSpec for INTERP0_CTRL_LANE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp0_ctrl_lane0::R](R) reader structure"]
impl crate::Readable for INTERP0_CTRL_LANE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp0_ctrl_lane0::W](W) writer structure"]
impl crate::Writable for INTERP0_CTRL_LANE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERP0_CTRL_LANE0 to value 0"]
impl crate::Resettable for INTERP0_CTRL_LANE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
