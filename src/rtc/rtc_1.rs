#[doc = "Reader of register RTC_1"]
pub type R = crate::R<u32, super::RTC_1>;
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u16, u16>;
#[doc = "Reader of field `MONTH`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DAY`"]
pub type DAY_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
}
