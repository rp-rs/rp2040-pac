#[doc = "Register `SSPPCELLID2` reader"]
pub type R = crate::R<SSPPCELLID2_SPEC>;
#[doc = "Register `SSPPCELLID2` writer"]
pub type W = crate::W<SSPPCELLID2_SPEC>;
#[doc = "Field `SSPPCELLID2` reader - These bits read back as 0x05"]
pub type SSPPCELLID2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn ssppcellid2(&self) -> SSPPCELLID2_R {
        SSPPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssppcellid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID2_SPEC;
impl crate::RegisterSpec for SSPPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid2::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssppcellid2::W`](W) writer structure"]
impl crate::Writable for SSPPCELLID2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPCELLID2 to value 0x05"]
impl crate::Resettable for SSPPCELLID2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
