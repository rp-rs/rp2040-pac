#[doc = "Register `SSPPCELLID1` reader"]
pub type R = crate::R<SSPPCELLID1_SPEC>;
#[doc = "Register `SSPPCELLID1` writer"]
pub type W = crate::W<SSPPCELLID1_SPEC>;
#[doc = "Field `SSPPCELLID1` reader - These bits read back as 0xF0"]
pub type SSPPCELLID1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn ssppcellid1(&self) -> SSPPCELLID1_R {
        SSPPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssppcellid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID1_SPEC;
impl crate::RegisterSpec for SSPPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid1::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssppcellid1::W`](W) writer structure"]
impl crate::Writable for SSPPCELLID1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPCELLID1 to value 0xf0"]
impl crate::Resettable for SSPPCELLID1_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
