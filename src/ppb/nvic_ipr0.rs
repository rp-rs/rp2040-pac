#[doc = "Register `NVIC_IPR0` reader"]
pub type R = crate::R<NVIC_IPR0_SPEC>;
#[doc = "Register `NVIC_IPR0` writer"]
pub type W = crate::W<NVIC_IPR0_SPEC>;
#[doc = "Field `IP_0` reader - Priority of interrupt 0"]
pub type IP_0_R = crate::FieldReader;
#[doc = "Field `IP_0` writer - Priority of interrupt 0"]
pub type IP_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_1` reader - Priority of interrupt 1"]
pub type IP_1_R = crate::FieldReader;
#[doc = "Field `IP_1` writer - Priority of interrupt 1"]
pub type IP_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_2` reader - Priority of interrupt 2"]
pub type IP_2_R = crate::FieldReader;
#[doc = "Field `IP_2` writer - Priority of interrupt 2"]
pub type IP_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_3` reader - Priority of interrupt 3"]
pub type IP_3_R = crate::FieldReader;
#[doc = "Field `IP_3` writer - Priority of interrupt 3"]
pub type IP_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 0"]
    #[inline(always)]
    pub fn ip_0(&self) -> IP_0_R {
        IP_0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 1"]
    #[inline(always)]
    pub fn ip_1(&self) -> IP_1_R {
        IP_1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 2"]
    #[inline(always)]
    pub fn ip_2(&self) -> IP_2_R {
        IP_2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 3"]
    #[inline(always)]
    pub fn ip_3(&self) -> IP_3_R {
        IP_3_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn ip_0(&mut self) -> IP_0_W<NVIC_IPR0_SPEC> {
        IP_0_W::new(self, 6)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn ip_1(&mut self) -> IP_1_W<NVIC_IPR0_SPEC> {
        IP_1_W::new(self, 14)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn ip_2(&mut self) -> IP_2_W<NVIC_IPR0_SPEC> {
        IP_2_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn ip_3(&mut self) -> IP_3_W<NVIC_IPR0_SPEC> {
        IP_3_W::new(self, 30)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR0_SPEC;
impl crate::RegisterSpec for NVIC_IPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr0::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr0::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR0 to value 0"]
impl crate::Resettable for NVIC_IPR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
