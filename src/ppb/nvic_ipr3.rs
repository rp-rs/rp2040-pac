#[doc = "Register `NVIC_IPR3` reader"]
pub type R = crate::R<NVIC_IPR3_SPEC>;
#[doc = "Register `NVIC_IPR3` writer"]
pub type W = crate::W<NVIC_IPR3_SPEC>;
#[doc = "Field `IP_12` reader - Priority of interrupt 12"]
pub type IP_12_R = crate::FieldReader;
#[doc = "Field `IP_12` writer - Priority of interrupt 12"]
pub type IP_12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_13` reader - Priority of interrupt 13"]
pub type IP_13_R = crate::FieldReader;
#[doc = "Field `IP_13` writer - Priority of interrupt 13"]
pub type IP_13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_14` reader - Priority of interrupt 14"]
pub type IP_14_R = crate::FieldReader;
#[doc = "Field `IP_14` writer - Priority of interrupt 14"]
pub type IP_14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_15` reader - Priority of interrupt 15"]
pub type IP_15_R = crate::FieldReader;
#[doc = "Field `IP_15` writer - Priority of interrupt 15"]
pub type IP_15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    pub fn ip_12(&self) -> IP_12_R {
        IP_12_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    pub fn ip_13(&self) -> IP_13_R {
        IP_13_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    pub fn ip_14(&self) -> IP_14_R {
        IP_14_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    pub fn ip_15(&self) -> IP_15_R {
        IP_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn ip_12(&mut self) -> IP_12_W<NVIC_IPR3_SPEC> {
        IP_12_W::new(self, 6)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn ip_13(&mut self) -> IP_13_W<NVIC_IPR3_SPEC> {
        IP_13_W::new(self, 14)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn ip_14(&mut self) -> IP_14_W<NVIC_IPR3_SPEC> {
        IP_14_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn ip_15(&mut self) -> IP_15_W<NVIC_IPR3_SPEC> {
        IP_15_W::new(self, 30)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR3_SPEC;
impl crate::RegisterSpec for NVIC_IPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr3::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr3::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR3 to value 0"]
impl crate::Resettable for NVIC_IPR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
