#[doc = "Register `NVIC_IPR2` reader"]
pub type R = crate::R<NVIC_IPR2_SPEC>;
#[doc = "Register `NVIC_IPR2` writer"]
pub type W = crate::W<NVIC_IPR2_SPEC>;
#[doc = "Field `IP_8` reader - Priority of interrupt 8"]
pub type IP_8_R = crate::FieldReader;
#[doc = "Field `IP_8` writer - Priority of interrupt 8"]
pub type IP_8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_9` reader - Priority of interrupt 9"]
pub type IP_9_R = crate::FieldReader;
#[doc = "Field `IP_9` writer - Priority of interrupt 9"]
pub type IP_9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_10` reader - Priority of interrupt 10"]
pub type IP_10_R = crate::FieldReader;
#[doc = "Field `IP_10` writer - Priority of interrupt 10"]
pub type IP_10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IP_11` reader - Priority of interrupt 11"]
pub type IP_11_R = crate::FieldReader;
#[doc = "Field `IP_11` writer - Priority of interrupt 11"]
pub type IP_11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn ip_8(&self) -> IP_8_R {
        IP_8_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    pub fn ip_9(&self) -> IP_9_R {
        IP_9_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    pub fn ip_10(&self) -> IP_10_R {
        IP_10_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    pub fn ip_11(&self) -> IP_11_R {
        IP_11_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn ip_8(&mut self) -> IP_8_W<NVIC_IPR2_SPEC> {
        IP_8_W::new(self, 6)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn ip_9(&mut self) -> IP_9_W<NVIC_IPR2_SPEC> {
        IP_9_W::new(self, 14)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn ip_10(&mut self) -> IP_10_W<NVIC_IPR2_SPEC> {
        IP_10_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn ip_11(&mut self) -> IP_11_W<NVIC_IPR2_SPEC> {
        IP_11_W::new(self, 30)
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

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR2_SPEC;
impl crate::RegisterSpec for NVIC_IPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr2::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr2::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR2 to value 0"]
impl crate::Resettable for NVIC_IPR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
