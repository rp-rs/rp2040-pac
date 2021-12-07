#[doc = "Register `EP_STALL_ARM` reader"]
pub struct R(crate::R<EP_STALL_ARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_STALL_ARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_STALL_ARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_STALL_ARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_STALL_ARM` writer"]
pub struct W(crate::W<EP_STALL_ARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_STALL_ARM_SPEC>;
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
impl From<crate::W<EP_STALL_ARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_STALL_ARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0_OUT` reader - "]
pub struct EP0_OUT_R(crate::FieldReader<bool, bool>);
impl EP0_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_OUT` writer - "]
pub struct EP0_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EP0_IN` reader - "]
pub struct EP0_IN_R(crate::FieldReader<bool, bool>);
impl EP0_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_IN` writer - "]
pub struct EP0_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_IN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&mut self) -> EP0_OUT_W {
        EP0_OUT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&mut self) -> EP0_IN_W {
        EP0_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ep_stall_arm](index.html) module"]
pub struct EP_STALL_ARM_SPEC;
impl crate::RegisterSpec for EP_STALL_ARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_stall_arm::R](R) reader structure"]
impl crate::Readable for EP_STALL_ARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_stall_arm::W](W) writer structure"]
impl crate::Writable for EP_STALL_ARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP_STALL_ARM to value 0"]
impl crate::Resettable for EP_STALL_ARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
