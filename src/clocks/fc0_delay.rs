#[doc = "Register `FC0_DELAY` reader"]
pub type R = crate::R<FC0_DELAY_SPEC>;
#[doc = "Register `FC0_DELAY` writer"]
pub type W = crate::W<FC0_DELAY_SPEC>;
#[doc = "Field `FC0_DELAY` reader - "]
pub type FC0_DELAY_R = crate::FieldReader;
#[doc = "Field `FC0_DELAY` writer - "]
pub type FC0_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fc0_delay(&self) -> FC0_DELAY_R {
        FC0_DELAY_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_delay(&mut self) -> FC0_DELAY_W<FC0_DELAY_SPEC> {
        FC0_DELAY_W::new(self, 0)
    }
}
#[doc = "Delays the start of frequency counting to allow the mux to settle  
 Delay is measured in multiples of the reference clock period  

You can [`read`](crate::Reg::read) this register and get [`fc0_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc0_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_DELAY_SPEC;
impl crate::RegisterSpec for FC0_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_delay::R`](R) reader structure"]
impl crate::Readable for FC0_DELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_delay::W`](W) writer structure"]
impl crate::Writable for FC0_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC0_DELAY to value 0x01"]
impl crate::Resettable for FC0_DELAY_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
