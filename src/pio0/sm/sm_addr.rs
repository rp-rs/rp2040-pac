#[doc = "Register `SM_ADDR` reader"]
pub type R = crate::R<SM_ADDR_SPEC>;
#[doc = "Field `SM0_ADDR` reader - "]
pub type SM0_ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm0_addr(&self) -> SM0_ADDR_R {
        SM0_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Current instruction address of state machine 0  

You can [`read`](crate::Reg::read) this register and get [`sm_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_ADDR_SPEC;
impl crate::RegisterSpec for SM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_addr::R`](R) reader structure"]
impl crate::Readable for SM_ADDR_SPEC {}
#[doc = "`reset()` method sets SM_ADDR to value 0"]
impl crate::Resettable for SM_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
