#[doc = "Reader of register FC0_STATUS"]
pub type R = crate::R<u32, super::FC0_STATUS>;
#[doc = "Reader of field `DIED`"]
pub type DIED_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAST`"]
pub type FAST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLOW`"]
pub type SLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAIL`"]
pub type FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAITING`"]
pub type WAITING_R = crate::R<bool, bool>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PASS`"]
pub type PASS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 28 - Test clock stopped during test"]
    #[inline(always)]
    pub fn died(&self) -> DIED_R {
        DIED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Test clock faster than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Test clock slower than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn slow(&self) -> SLOW_R {
        SLOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Test failed"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Waiting for test clock to start"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Test running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Test complete"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Test passed"]
    #[inline(always)]
    pub fn pass(&self) -> PASS_R {
        PASS_R::new((self.bits & 0x01) != 0)
    }
}
