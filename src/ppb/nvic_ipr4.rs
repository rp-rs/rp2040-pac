#[doc = "Register `NVIC_IPR4` reader"]
pub type R = crate::R<NVIC_IPR4_SPEC>;
#[doc = "Register `NVIC_IPR4` writer"]
pub type W = crate::W<NVIC_IPR4_SPEC>;
#[doc = "Field `IP_16` reader - Priority of interrupt 16"]
pub type IP_16_R = crate::FieldReader;
#[doc = "Field `IP_16` writer - Priority of interrupt 16"]
pub type IP_16_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_17` reader - Priority of interrupt 17"]
pub type IP_17_R = crate::FieldReader;
#[doc = "Field `IP_17` writer - Priority of interrupt 17"]
pub type IP_17_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_18` reader - Priority of interrupt 18"]
pub type IP_18_R = crate::FieldReader;
#[doc = "Field `IP_18` writer - Priority of interrupt 18"]
pub type IP_18_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_19` reader - Priority of interrupt 19"]
pub type IP_19_R = crate::FieldReader;
#[doc = "Field `IP_19` writer - Priority of interrupt 19"]
pub type IP_19_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 16"]
    #[inline(always)]
    pub fn ip_16(&self) -> IP_16_R {
        IP_16_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 17"]
    #[inline(always)]
    pub fn ip_17(&self) -> IP_17_R {
        IP_17_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 18"]
    #[inline(always)]
    pub fn ip_18(&self) -> IP_18_R {
        IP_18_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 19"]
    #[inline(always)]
    pub fn ip_19(&self) -> IP_19_R {
        IP_19_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn ip_16(&mut self) -> IP_16_W<NVIC_IPR4_SPEC> {
        IP_16_W::new(self, 6)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn ip_17(&mut self) -> IP_17_W<NVIC_IPR4_SPEC> {
        IP_17_W::new(self, 14)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn ip_18(&mut self) -> IP_18_W<NVIC_IPR4_SPEC> {
        IP_18_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn ip_19(&mut self) -> IP_19_W<NVIC_IPR4_SPEC> {
        IP_19_W::new(self, 30)
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
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR4_SPEC;
impl crate::RegisterSpec for NVIC_IPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr4::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr4::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR4 to value 0"]
impl crate::Resettable for NVIC_IPR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
