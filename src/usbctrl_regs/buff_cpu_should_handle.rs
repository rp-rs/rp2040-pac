#[doc = "Register `BUFF_CPU_SHOULD_HANDLE` reader"]
pub type R = crate::R<BUFF_CPU_SHOULD_HANDLE_SPEC>;
#[doc = "Field `EP0_IN` reader - "]
pub type EP0_IN_R = crate::BitReader;
#[doc = "Field `EP0_OUT` reader - "]
pub type EP0_OUT_R = crate::BitReader;
#[doc = "Field `EP1_IN` reader - "]
pub type EP1_IN_R = crate::BitReader;
#[doc = "Field `EP1_OUT` reader - "]
pub type EP1_OUT_R = crate::BitReader;
#[doc = "Field `EP2_IN` reader - "]
pub type EP2_IN_R = crate::BitReader;
#[doc = "Field `EP2_OUT` reader - "]
pub type EP2_OUT_R = crate::BitReader;
#[doc = "Field `EP3_IN` reader - "]
pub type EP3_IN_R = crate::BitReader;
#[doc = "Field `EP3_OUT` reader - "]
pub type EP3_OUT_R = crate::BitReader;
#[doc = "Field `EP4_IN` reader - "]
pub type EP4_IN_R = crate::BitReader;
#[doc = "Field `EP4_OUT` reader - "]
pub type EP4_OUT_R = crate::BitReader;
#[doc = "Field `EP5_IN` reader - "]
pub type EP5_IN_R = crate::BitReader;
#[doc = "Field `EP5_OUT` reader - "]
pub type EP5_OUT_R = crate::BitReader;
#[doc = "Field `EP6_IN` reader - "]
pub type EP6_IN_R = crate::BitReader;
#[doc = "Field `EP6_OUT` reader - "]
pub type EP6_OUT_R = crate::BitReader;
#[doc = "Field `EP7_IN` reader - "]
pub type EP7_IN_R = crate::BitReader;
#[doc = "Field `EP7_OUT` reader - "]
pub type EP7_OUT_R = crate::BitReader;
#[doc = "Field `EP8_IN` reader - "]
pub type EP8_IN_R = crate::BitReader;
#[doc = "Field `EP8_OUT` reader - "]
pub type EP8_OUT_R = crate::BitReader;
#[doc = "Field `EP9_IN` reader - "]
pub type EP9_IN_R = crate::BitReader;
#[doc = "Field `EP9_OUT` reader - "]
pub type EP9_OUT_R = crate::BitReader;
#[doc = "Field `EP10_IN` reader - "]
pub type EP10_IN_R = crate::BitReader;
#[doc = "Field `EP10_OUT` reader - "]
pub type EP10_OUT_R = crate::BitReader;
#[doc = "Field `EP11_IN` reader - "]
pub type EP11_IN_R = crate::BitReader;
#[doc = "Field `EP11_OUT` reader - "]
pub type EP11_OUT_R = crate::BitReader;
#[doc = "Field `EP12_IN` reader - "]
pub type EP12_IN_R = crate::BitReader;
#[doc = "Field `EP12_OUT` reader - "]
pub type EP12_OUT_R = crate::BitReader;
#[doc = "Field `EP13_IN` reader - "]
pub type EP13_IN_R = crate::BitReader;
#[doc = "Field `EP13_OUT` reader - "]
pub type EP13_OUT_R = crate::BitReader;
#[doc = "Field `EP14_IN` reader - "]
pub type EP14_IN_R = crate::BitReader;
#[doc = "Field `EP14_OUT` reader - "]
pub type EP14_OUT_R = crate::BitReader;
#[doc = "Field `EP15_IN` reader - "]
pub type EP15_IN_R = crate::BitReader;
#[doc = "Field `EP15_OUT` reader - "]
pub type EP15_OUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep1_in(&self) -> EP1_IN_R {
        EP1_IN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep1_out(&self) -> EP1_OUT_R {
        EP1_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep2_in(&self) -> EP2_IN_R {
        EP2_IN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep2_out(&self) -> EP2_OUT_R {
        EP2_OUT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep3_in(&self) -> EP3_IN_R {
        EP3_IN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep3_out(&self) -> EP3_OUT_R {
        EP3_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep4_in(&self) -> EP4_IN_R {
        EP4_IN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep4_out(&self) -> EP4_OUT_R {
        EP4_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep5_in(&self) -> EP5_IN_R {
        EP5_IN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep5_out(&self) -> EP5_OUT_R {
        EP5_OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep6_in(&self) -> EP6_IN_R {
        EP6_IN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep6_out(&self) -> EP6_OUT_R {
        EP6_OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep7_in(&self) -> EP7_IN_R {
        EP7_IN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep7_out(&self) -> EP7_OUT_R {
        EP7_OUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep8_in(&self) -> EP8_IN_R {
        EP8_IN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep8_out(&self) -> EP8_OUT_R {
        EP8_OUT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep9_in(&self) -> EP9_IN_R {
        EP9_IN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep9_out(&self) -> EP9_OUT_R {
        EP9_OUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep10_in(&self) -> EP10_IN_R {
        EP10_IN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep10_out(&self) -> EP10_OUT_R {
        EP10_OUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep11_in(&self) -> EP11_IN_R {
        EP11_IN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep11_out(&self) -> EP11_OUT_R {
        EP11_OUT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ep12_in(&self) -> EP12_IN_R {
        EP12_IN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ep12_out(&self) -> EP12_OUT_R {
        EP12_OUT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ep13_in(&self) -> EP13_IN_R {
        EP13_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ep13_out(&self) -> EP13_OUT_R {
        EP13_OUT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ep14_in(&self) -> EP14_IN_R {
        EP14_IN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ep14_out(&self) -> EP14_OUT_R {
        EP14_OUT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep15_in(&self) -> EP15_IN_R {
        EP15_IN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep15_out(&self) -> EP15_OUT_R {
        EP15_OUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered.  

You can [`read`](crate::Reg::read) this register and get [`buff_cpu_should_handle::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFF_CPU_SHOULD_HANDLE_SPEC;
impl crate::RegisterSpec for BUFF_CPU_SHOULD_HANDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buff_cpu_should_handle::R`](R) reader structure"]
impl crate::Readable for BUFF_CPU_SHOULD_HANDLE_SPEC {}
#[doc = "`reset()` method sets BUFF_CPU_SHOULD_HANDLE to value 0"]
impl crate::Resettable for BUFF_CPU_SHOULD_HANDLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
