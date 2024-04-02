#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDR_SPEC>;
#[doc = "Field `IDCODE` reader - Peripheral dentification code"]
pub type IDCODE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral dentification code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(self.bits)
    }
}
#[doc = "Identification register  

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDR_SPEC {}
#[doc = "`reset()` method sets IDR to value 0x5153_5049"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: u32 = 0x5153_5049;
}
