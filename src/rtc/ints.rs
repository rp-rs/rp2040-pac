#[doc = "Reader of register INTS"]
pub type R = crate::R<u32, super::INTS>;
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x01) != 0)
    }
}
