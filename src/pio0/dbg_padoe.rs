#[doc = "Register `DBG_PADOE` reader"]
pub type R = crate::R<DBG_PADOE_SPEC>;
#[doc = "Register `DBG_PADOE` writer"]
pub type W = crate::W<DBG_PADOE_SPEC>;
#[doc = "Field `DBG_PADOE` reader - "]
pub type DBG_PADOE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dbg_padoe(&self) -> DBG_PADOE_R {
        DBG_PADOE_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_padoe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_padoe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_PADOE_SPEC;
impl crate::RegisterSpec for DBG_PADOE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_padoe::R`](R) reader structure"]
impl crate::Readable for DBG_PADOE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_padoe::W`](W) writer structure"]
impl crate::Writable for DBG_PADOE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_PADOE to value 0"]
impl crate::Resettable for DBG_PADOE_SPEC {
    const RESET_VALUE: u32 = 0;
}
