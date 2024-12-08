#[doc = "Register `SSPPERIPHID2` reader"]
pub type R = crate::R<SSPPERIPHID2_SPEC>;
#[doc = "Register `SSPPERIPHID2` writer"]
pub type W = crate::W<SSPPERIPHID2_SPEC>;
#[doc = "Field `DESIGNER1` reader - These bits read back as 0x4"]
pub type DESIGNER1_R = crate::FieldReader;
#[doc = "Field `REVISION` reader - These bits return the peripheral revision"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> DESIGNER1_R {
        DESIGNER1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits return the peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspperiphid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPERIPHID2_SPEC;
impl crate::RegisterSpec for SSPPERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspperiphid2::R`](R) reader structure"]
impl crate::Readable for SSPPERIPHID2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspperiphid2::W`](W) writer structure"]
impl crate::Writable for SSPPERIPHID2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPERIPHID2 to value 0x34"]
impl crate::Resettable for SSPPERIPHID2_SPEC {
    const RESET_VALUE: u32 = 0x34;
}
