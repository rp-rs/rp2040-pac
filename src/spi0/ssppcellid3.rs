#[doc = "Register `SSPPCELLID3` reader"]
pub type R = crate::R<SSPPCELLID3_SPEC>;
#[doc = "Register `SSPPCELLID3` writer"]
pub type W = crate::W<SSPPCELLID3_SPEC>;
#[doc = "Field `SSPPCELLID3` reader - These bits read back as 0xB1"]
pub type SSPPCELLID3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn ssppcellid3(&self) -> SSPPCELLID3_R {
        SSPPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssppcellid3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID3_SPEC;
impl crate::RegisterSpec for SSPPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid3::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssppcellid3::W`](W) writer structure"]
impl crate::Writable for SSPPCELLID3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPCELLID3 to value 0xb1"]
impl crate::Resettable for SSPPCELLID3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
