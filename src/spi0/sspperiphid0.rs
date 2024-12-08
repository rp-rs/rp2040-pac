#[doc = "Register `SSPPERIPHID0` reader"]
pub type R = crate::R<SSPPERIPHID0_SPEC>;
#[doc = "Register `SSPPERIPHID0` writer"]
pub type W = crate::W<SSPPERIPHID0_SPEC>;
#[doc = "Field `PARTNUMBER0` reader - These bits read back as 0x22"]
pub type PARTNUMBER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x22"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspperiphid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPERIPHID0_SPEC;
impl crate::RegisterSpec for SSPPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspperiphid0::R`](R) reader structure"]
impl crate::Readable for SSPPERIPHID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspperiphid0::W`](W) writer structure"]
impl crate::Writable for SSPPERIPHID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPERIPHID0 to value 0x22"]
impl crate::Resettable for SSPPERIPHID0_SPEC {
    const RESET_VALUE: u32 = 0x22;
}
