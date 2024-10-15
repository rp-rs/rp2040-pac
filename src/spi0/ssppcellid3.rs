#[doc = "Register `SSPPCELLID3` reader"]
pub type R = crate::R<SSPPCELLID3_SPEC>;
#[doc = "Field `SSPPCELLID3` reader - These bits read back as 0xB1"]
pub type SSPPCELLID3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn ssppcellid3(&self) -> SSPPCELLID3_R {
        SSPPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::Reg::read) this register and get [`ssppcellid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID3_SPEC;
impl crate::RegisterSpec for SSPPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid3::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID3_SPEC {}
#[doc = "`reset()` method sets SSPPCELLID3 to value 0xb1"]
impl crate::Resettable for SSPPCELLID3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
