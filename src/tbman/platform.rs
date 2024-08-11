#[doc = "Register `PLATFORM` reader"]
pub type R = crate::R<PLATFORM_SPEC>;
#[doc = "Register `PLATFORM` writer"]
pub type W = crate::W<PLATFORM_SPEC>;
#[doc = "Field `ASIC` reader - Indicates the platform is an ASIC"]
pub type ASIC_R = crate::BitReader;
#[doc = "Field `FPGA` reader - Indicates the platform is an FPGA"]
pub type FPGA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the platform is an ASIC"]
    #[inline(always)]
    pub fn asic(&self) -> ASIC_R {
        ASIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the platform is an FPGA"]
    #[inline(always)]
    pub fn fpga(&self) -> FPGA_R {
        FPGA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "Indicates the type of platform in use  

You can [`read`](crate::generic::Reg::read) this register and get [`platform::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`platform::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLATFORM_SPEC;
impl crate::RegisterSpec for PLATFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`platform::R`](R) reader structure"]
impl crate::Readable for PLATFORM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`platform::W`](W) writer structure"]
impl crate::Writable for PLATFORM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLATFORM to value 0x05"]
impl crate::Resettable for PLATFORM_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
