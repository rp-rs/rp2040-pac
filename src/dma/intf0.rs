#[doc = "Register `INTF0` reader"]
pub type R = crate::R<INTF0_SPEC>;
#[doc = "Register `INTF0` writer"]
pub type W = crate::W<INTF0_SPEC>;
#[doc = "Field `INTF0` reader - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
pub type INTF0_R = crate::FieldReader<u16>;
#[doc = "Field `INTF0` writer - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
pub type INTF0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf0(&self) -> INTF0_R {
        INTF0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intf0(&mut self) -> INTF0_W<INTF0_SPEC> {
        INTF0_W::new(self, 0)
    }
}
#[doc = "Force Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF0_SPEC;
impl crate::RegisterSpec for INTF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf0::R`](R) reader structure"]
impl crate::Readable for INTF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf0::W`](W) writer structure"]
impl crate::Writable for INTF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF0 to value 0"]
impl crate::Resettable for INTF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
