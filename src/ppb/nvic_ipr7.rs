#[doc = "Register `NVIC_IPR7` reader"]
pub type R = crate::R<NVIC_IPR7_SPEC>;
#[doc = "Register `NVIC_IPR7` writer"]
pub type W = crate::W<NVIC_IPR7_SPEC>;
#[doc = "Field `IP_28` reader - Priority of interrupt 28"]
pub type IP_28_R = crate::FieldReader;
#[doc = "Field `IP_28` writer - Priority of interrupt 28"]
pub type IP_28_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_29` reader - Priority of interrupt 29"]
pub type IP_29_R = crate::FieldReader;
#[doc = "Field `IP_29` writer - Priority of interrupt 29"]
pub type IP_29_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_30` reader - Priority of interrupt 30"]
pub type IP_30_R = crate::FieldReader;
#[doc = "Field `IP_30` writer - Priority of interrupt 30"]
pub type IP_30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_31` reader - Priority of interrupt 31"]
pub type IP_31_R = crate::FieldReader;
#[doc = "Field `IP_31` writer - Priority of interrupt 31"]
pub type IP_31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 28"]
    #[inline(always)]
    pub fn ip_28(&self) -> IP_28_R {
        IP_28_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 29"]
    #[inline(always)]
    pub fn ip_29(&self) -> IP_29_R {
        IP_29_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 30"]
    #[inline(always)]
    pub fn ip_30(&self) -> IP_30_R {
        IP_30_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 31"]
    #[inline(always)]
    pub fn ip_31(&self) -> IP_31_R {
        IP_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn ip_28(&mut self) -> IP_28_W<NVIC_IPR7_SPEC, 6> {
        IP_28_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn ip_29(&mut self) -> IP_29_W<NVIC_IPR7_SPEC, 14> {
        IP_29_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn ip_30(&mut self) -> IP_30_W<NVIC_IPR7_SPEC, 22> {
        IP_30_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn ip_31(&mut self) -> IP_31_W<NVIC_IPR7_SPEC, 30> {
        IP_31_W::new(self)
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

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR7_SPEC;
impl crate::RegisterSpec for NVIC_IPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr7::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr7::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR7 to value 0"]
impl crate::Resettable for NVIC_IPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
