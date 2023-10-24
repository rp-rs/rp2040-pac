#[doc = "Register `EP_STALL_ARM` reader"]
pub type R = crate::R<EP_STALL_ARM_SPEC>;
#[doc = "Register `EP_STALL_ARM` writer"]
pub type W = crate::W<EP_STALL_ARM_SPEC>;
#[doc = "Field `EP0_IN` reader - "]
pub type EP0_IN_R = crate::BitReader;
#[doc = "Field `EP0_IN` writer - "]
pub type EP0_IN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP0_OUT` reader - "]
pub type EP0_OUT_R = crate::BitReader;
#[doc = "Field `EP0_OUT` writer - "]
pub type EP0_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn ep0_in(&mut self) -> EP0_IN_W<EP_STALL_ARM_SPEC, 0> {
        EP0_IN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_out(&mut self) -> EP0_OUT_W<EP_STALL_ARM_SPEC, 1> {
        EP0_OUT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received.  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_stall_arm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_stall_arm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_STALL_ARM_SPEC;
impl crate::RegisterSpec for EP_STALL_ARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_stall_arm::R`](R) reader structure"]
impl crate::Readable for EP_STALL_ARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_stall_arm::W`](W) writer structure"]
impl crate::Writable for EP_STALL_ARM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_STALL_ARM to value 0"]
impl crate::Resettable for EP_STALL_ARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
