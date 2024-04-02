#[doc = "Register `SSPPCELLID0` reader"]
pub type R = crate::R<SSPPCELLID0_SPEC>;
#[doc = "Field `SSPPCELLID0` reader - These bits read back as 0x0D"]
pub type SSPPCELLID0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn ssppcellid0(&self) -> SSPPCELLID0_R {
        SSPPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::Reg::read) this register and get [`ssppcellid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPCELLID0_SPEC;
impl crate::RegisterSpec for SSPPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssppcellid0::R`](R) reader structure"]
impl crate::Readable for SSPPCELLID0_SPEC {}
#[doc = "`reset()` method sets SSPPCELLID0 to value 0x0d"]
impl crate::Resettable for SSPPCELLID0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
