#[doc = "Register `SSPPCELLID1` reader"]
pub type R = crate::R<SSPPCELLID1_SPEC>;
#[doc = "Field `SSPPCELLID1` reader - These bits read back as 0xF0"]
pub type SSPPCELLID1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn ssppcellid1(&self) -> SSPPCELLID1_R {
        SSPPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::Reg::read) this register and get [`ssppcellid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID1_SPEC;
impl crate::RegisterSpec for SSPPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid1::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID1_SPEC {}
#[doc = "`reset()` method sets SSPPCELLID1 to value 0xf0"]
impl crate::Resettable for SSPPCELLID1_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
