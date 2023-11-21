#[doc = "Register `PLATFORM` reader"]
pub type R = crate::R<PLATFORM_SPEC>;
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
#[doc = "Indicates the type of platform in use  

You can [`read`](crate::generic::Reg::read) this register and get [`platform::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLATFORM_SPEC;
impl crate::RegisterSpec for PLATFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`platform::R`](R) reader structure"]
impl crate::Readable for PLATFORM_SPEC {}
#[doc = "`reset()` method sets PLATFORM to value 0x05"]
impl crate::Resettable for PLATFORM_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
