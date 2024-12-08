#[doc = "Register `UARTPERIPHID2` reader"]
pub type R = crate::R<UARTPERIPHID2_SPEC>;
#[doc = "Register `UARTPERIPHID2` writer"]
pub type W = crate::W<UARTPERIPHID2_SPEC>;
#[doc = "Field `DESIGNER1` reader - These bits read back as 0x4"]
pub type DESIGNER1_R = crate::FieldReader;
#[doc = "Field `REVISION` reader - This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> DESIGNER1_R {
        DESIGNER1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "UARTPeriphID2 Register  

You can [`read`](crate::generic::Reg::read) this register and get [`uartperiphid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartperiphid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPERIPHID2_SPEC;
impl crate::RegisterSpec for UARTPERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartperiphid2::R`](R) reader structure"]
impl crate::Readable for UARTPERIPHID2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartperiphid2::W`](W) writer structure"]
impl crate::Writable for UARTPERIPHID2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTPERIPHID2 to value 0x34"]
impl crate::Resettable for UARTPERIPHID2_SPEC {
    const RESET_VALUE: u32 = 0x34;
}
