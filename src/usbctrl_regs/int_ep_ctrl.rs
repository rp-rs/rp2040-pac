#[doc = "Register `INT_EP_CTRL` reader"]
pub type R = crate::R<INT_EP_CTRL_SPEC>;
#[doc = "Register `INT_EP_CTRL` writer"]
pub type W = crate::W<INT_EP_CTRL_SPEC>;
#[doc = "Field `INT_EP_ACTIVE` reader - Host: Enable interrupt endpoint 1 -> 15"]
pub type INT_EP_ACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `INT_EP_ACTIVE` writer - Host: Enable interrupt endpoint 1 -> 15"]
pub type INT_EP_ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 1:15 - Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub fn int_ep_active(&self) -> INT_EP_ACTIVE_R {
        INT_EP_ACTIVE_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    #[must_use]
    pub fn int_ep_active(&mut self) -> INT_EP_ACTIVE_W<INT_EP_CTRL_SPEC> {
        INT_EP_ACTIVE_W::new(self, 1)
    }
}
#[doc = "interrupt endpoint control register  

You can [`read`](crate::Reg::read) this register and get [`int_ep_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ep_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EP_CTRL_SPEC;
impl crate::RegisterSpec for INT_EP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ep_ctrl::R`](R) reader structure"]
impl crate::Readable for INT_EP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ep_ctrl::W`](W) writer structure"]
impl crate::Writable for INT_EP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EP_CTRL to value 0"]
impl crate::Resettable for INT_EP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
