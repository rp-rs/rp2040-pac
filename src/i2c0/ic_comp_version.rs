#[doc = "Register `IC_COMP_VERSION` reader"]
pub type R = crate::R<IC_COMP_VERSION_SPEC>;
#[doc = "Register `IC_COMP_VERSION` writer"]
pub type W = crate::W<IC_COMP_VERSION_SPEC>;
#[doc = "Field `IC_COMP_VERSION` reader - "]
pub type IC_COMP_VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic_comp_version(&self) -> IC_COMP_VERSION_R {
        IC_COMP_VERSION_R::new(self.bits)
    }
}
impl W {}
#[doc = "I2C Component Version Register  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_comp_version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_comp_version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_COMP_VERSION_SPEC;
impl crate::RegisterSpec for IC_COMP_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_comp_version::R`](R) reader structure"]
impl crate::Readable for IC_COMP_VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_comp_version::W`](W) writer structure"]
impl crate::Writable for IC_COMP_VERSION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_COMP_VERSION to value 0x3230_312a"]
impl crate::Resettable for IC_COMP_VERSION_SPEC {
    const RESET_VALUE: u32 = 0x3230_312a;
}
