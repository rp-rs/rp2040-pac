#[doc = "Register `ARMED` reader"]
pub type R = crate::R<ARMED_SPEC>;
#[doc = "Register `ARMED` writer"]
pub type W = crate::W<ARMED_SPEC>;
#[doc = "Field `ARMED` reader - "]
pub type ARMED_R = crate::FieldReader;
#[doc = "Field `ARMED` writer - "]
pub type ARMED_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn armed(&self) -> ARMED_R {
        ARMED_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn armed(&mut self) -> ARMED_W<ARMED_SPEC> {
        ARMED_W::new(self, 0)
    }
}
#[doc = "Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire.  

You can [`read`](crate::Reg::read) this register and get [`armed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`armed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARMED_SPEC;
impl crate::RegisterSpec for ARMED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`armed::R`](R) reader structure"]
impl crate::Readable for ARMED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`armed::W`](W) writer structure"]
impl crate::Writable for ARMED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets ARMED to value 0"]
impl crate::Resettable for ARMED_SPEC {
    const RESET_VALUE: u32 = 0;
}
