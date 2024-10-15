#[doc = "Register `FC0_STATUS` reader"]
pub type R = crate::R<FC0_STATUS_SPEC>;
#[doc = "Field `PASS` reader - Test passed"]
pub type PASS_R = crate::BitReader;
#[doc = "Field `DONE` reader - Test complete"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `RUNNING` reader - Test running"]
pub type RUNNING_R = crate::BitReader;
#[doc = "Field `WAITING` reader - Waiting for test clock to start"]
pub type WAITING_R = crate::BitReader;
#[doc = "Field `FAIL` reader - Test failed"]
pub type FAIL_R = crate::BitReader;
#[doc = "Field `SLOW` reader - Test clock slower than expected, only valid when status_done=1"]
pub type SLOW_R = crate::BitReader;
#[doc = "Field `FAST` reader - Test clock faster than expected, only valid when status_done=1"]
pub type FAST_R = crate::BitReader;
#[doc = "Field `DIED` reader - Test clock stopped during test"]
pub type DIED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Test passed"]
    #[inline(always)]
    pub fn pass(&self) -> PASS_R {
        PASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Test complete"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Test running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Waiting for test clock to start"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Test failed"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Test clock slower than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn slow(&self) -> SLOW_R {
        SLOW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Test clock faster than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Test clock stopped during test"]
    #[inline(always)]
    pub fn died(&self) -> DIED_R {
        DIED_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Frequency counter status  

You can [`read`](crate::Reg::read) this register and get [`fc0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_STATUS_SPEC;
impl crate::RegisterSpec for FC0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_status::R`](R) reader structure"]
impl crate::Readable for FC0_STATUS_SPEC {}
#[doc = "`reset()` method sets FC0_STATUS to value 0"]
impl crate::Resettable for FC0_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
