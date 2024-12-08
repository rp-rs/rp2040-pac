#[doc = "Register `UARTPCELLID3` reader"]
pub type R = crate::R<UARTPCELLID3_SPEC>;
#[doc = "Register `UARTPCELLID3` writer"]
pub type W = crate::W<UARTPCELLID3_SPEC>;
#[doc = "Field `UARTPCELLID3` reader - These bits read back as 0xB1"]
pub type UARTPCELLID3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn uartpcellid3(&self) -> UARTPCELLID3_R {
        UARTPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "UARTPCellID3 Register  

You can [`read`](crate::generic::Reg::read) this register and get [`uartpcellid3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartpcellid3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID3_SPEC;
impl crate::RegisterSpec for UARTPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid3::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartpcellid3::W`](W) writer structure"]
impl crate::Writable for UARTPCELLID3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTPCELLID3 to value 0xb1"]
impl crate::Resettable for UARTPCELLID3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
