#[doc = "Register `SSPPCELLID2` reader"]
pub type R = crate::R<SSPPCELLID2_SPEC>;
#[doc = "Field `SSPPCELLID2` reader - These bits read back as 0x05"]
pub type SSPPCELLID2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn ssppcellid2(&self) -> SSPPCELLID2_R {
        SSPPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::Reg::read) this register and get [`ssppcellid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID2_SPEC;
impl crate::RegisterSpec for SSPPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid2::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID2_SPEC {}
#[doc = "`reset()` method sets SSPPCELLID2 to value 0x05"]
impl crate::Resettable for SSPPCELLID2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
