#[doc = "Register `SSPPCELLID0` reader"]
pub type R = crate::R<SSPPCELLID0_SPEC>;
#[doc = "Register `SSPPCELLID0` writer"]
pub type W = crate::W<SSPPCELLID0_SPEC>;
#[doc = "Field `SSPPCELLID0` reader - These bits read back as 0x0D"]
pub type SSPPCELLID0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn ssppcellid0(&self) -> SSPPCELLID0_R {
        SSPPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssppcellid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID0_SPEC;
impl crate::RegisterSpec for SSPPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid0::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssppcellid0::W`](W) writer structure"]
impl crate::Writable for SSPPCELLID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPPCELLID0 to value 0x0d"]
impl crate::Resettable for SSPPCELLID0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
