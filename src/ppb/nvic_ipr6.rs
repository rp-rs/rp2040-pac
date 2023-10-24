#[doc = "Register `NVIC_IPR6` reader"]
pub type R = crate::R<NVIC_IPR6_SPEC>;
#[doc = "Register `NVIC_IPR6` writer"]
pub type W = crate::W<NVIC_IPR6_SPEC>;
#[doc = "Field `IP_24` reader - Priority of interrupt 24"]
pub type IP_24_R = crate::FieldReader;
#[doc = "Field `IP_24` writer - Priority of interrupt 24"]
pub type IP_24_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_25` reader - Priority of interrupt 25"]
pub type IP_25_R = crate::FieldReader;
#[doc = "Field `IP_25` writer - Priority of interrupt 25"]
pub type IP_25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_26` reader - Priority of interrupt 26"]
pub type IP_26_R = crate::FieldReader;
#[doc = "Field `IP_26` writer - Priority of interrupt 26"]
pub type IP_26_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_27` reader - Priority of interrupt 27"]
pub type IP_27_R = crate::FieldReader;
#[doc = "Field `IP_27` writer - Priority of interrupt 27"]
pub type IP_27_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 24"]
    #[inline(always)]
    pub fn ip_24(&self) -> IP_24_R {
        IP_24_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 25"]
    #[inline(always)]
    pub fn ip_25(&self) -> IP_25_R {
        IP_25_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 26"]
    #[inline(always)]
    pub fn ip_26(&self) -> IP_26_R {
        IP_26_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 27"]
    #[inline(always)]
    pub fn ip_27(&self) -> IP_27_R {
        IP_27_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn ip_24(&mut self) -> IP_24_W<NVIC_IPR6_SPEC, 6> {
        IP_24_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn ip_25(&mut self) -> IP_25_W<NVIC_IPR6_SPEC, 14> {
        IP_25_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn ip_26(&mut self) -> IP_26_W<NVIC_IPR6_SPEC, 22> {
        IP_26_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn ip_27(&mut self) -> IP_27_W<NVIC_IPR6_SPEC, 30> {
        IP_27_W::new(self)
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

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR6_SPEC;
impl crate::RegisterSpec for NVIC_IPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr6::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr6::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR6 to value 0"]
impl crate::Resettable for NVIC_IPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
