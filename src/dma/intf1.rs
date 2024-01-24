#[doc = "Register `INTF1` reader"]
pub type R = crate::R<INTF1_SPEC>;
#[doc = "Register `INTF1` writer"]
pub type W = crate::W<INTF1_SPEC>;
#[doc = "Field `INTF1` reader - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
pub type INTF1_R = crate::FieldReader<u16>;
#[doc = "Field `INTF1` writer - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
pub type INTF1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf1(&self) -> INTF1_R {
        INTF1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intf1(&mut self) -> INTF1_W<INTF1_SPEC> {
        INTF1_W::new(self, 0)
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
#[doc = "Force Interrupts for IRQ 1  

You can [`read`](crate::generic::Reg::read) this register and get [`intf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF1_SPEC;
impl crate::RegisterSpec for INTF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf1::R`](R) reader structure"]
impl crate::Readable for INTF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf1::W`](W) writer structure"]
impl crate::Writable for INTF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF1 to value 0"]
impl crate::Resettable for INTF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
