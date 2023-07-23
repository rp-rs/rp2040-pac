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
#[doc = "Field `EP0_IN` reader - "]
pub type EP0_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP0_IN` writer - "]
pub type EP0_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_STALL_ARM_SPEC, bool, O>;
#[doc = "Field `EP0_OUT` reader - "]
pub type EP0_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP0_OUT` writer - "]
pub type EP0_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_STALL_ARM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_in(&mut self) -> EP0_IN_W<0> {
        EP0_IN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_out(&mut self) -> EP0_OUT_W<1> {
        EP0_OUT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_STALL_ARM to value 0"]
impl crate::Resettable for EP_STALL_ARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
