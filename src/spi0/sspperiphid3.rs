#[doc = "Register `SSPPERIPHID3` reader"]
pub type R = crate::R<SSPPERIPHID3_SPEC>;
#[doc = "Register `SSPPERIPHID3` writer"]
pub type W = crate::W<SSPPERIPHID3_SPEC>;
#[doc = "Field `CONFIGURATION` reader - These bits read back as 0x00"]
pub type CONFIGURATION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspperiphid3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPERIPHID3_SPEC;
impl crate::RegisterSpec for SSPPERIPHID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspperiphid3::R`](R) reader structure"]
impl crate::Readable for SSPPERIPHID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspperiphid3::W`](W) writer structure"]
impl crate::Writable for SSPPERIPHID3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPERIPHID3 to value 0"]
impl crate::Resettable for SSPPERIPHID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
