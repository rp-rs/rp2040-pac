#[doc = "Reader of register REASON"]
pub type R = crate::R<u32, super::REASON>;
#[doc = "Reader of field `FORCE`"]
pub type FORCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x01) != 0)
    }
}
