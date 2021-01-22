#[doc = "Reader of register PROC0_INTS3"]
pub type R = crate::R<u32, super::PROC0_INTS3>;
#[doc = "Reader of field `GPIO29_EDGE_HIGH`"]
pub type GPIO29_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO29_EDGE_LOW`"]
pub type GPIO29_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO29_LEVEL_HIGH`"]
pub type GPIO29_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO29_LEVEL_LOW`"]
pub type GPIO29_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO28_EDGE_HIGH`"]
pub type GPIO28_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO28_EDGE_LOW`"]
pub type GPIO28_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO28_LEVEL_HIGH`"]
pub type GPIO28_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO28_LEVEL_LOW`"]
pub type GPIO28_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO27_EDGE_HIGH`"]
pub type GPIO27_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO27_EDGE_LOW`"]
pub type GPIO27_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO27_LEVEL_HIGH`"]
pub type GPIO27_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO27_LEVEL_LOW`"]
pub type GPIO27_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO26_EDGE_HIGH`"]
pub type GPIO26_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO26_EDGE_LOW`"]
pub type GPIO26_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO26_LEVEL_HIGH`"]
pub type GPIO26_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO26_LEVEL_LOW`"]
pub type GPIO26_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO25_EDGE_HIGH`"]
pub type GPIO25_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO25_EDGE_LOW`"]
pub type GPIO25_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO25_LEVEL_HIGH`"]
pub type GPIO25_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO25_LEVEL_LOW`"]
pub type GPIO25_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO24_EDGE_HIGH`"]
pub type GPIO24_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO24_EDGE_LOW`"]
pub type GPIO24_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO24_LEVEL_HIGH`"]
pub type GPIO24_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO24_LEVEL_LOW`"]
pub type GPIO24_LEVEL_LOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio29_edge_high(&self) -> GPIO29_EDGE_HIGH_R {
        GPIO29_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio29_edge_low(&self) -> GPIO29_EDGE_LOW_R {
        GPIO29_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio29_level_high(&self) -> GPIO29_LEVEL_HIGH_R {
        GPIO29_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio29_level_low(&self) -> GPIO29_LEVEL_LOW_R {
        GPIO29_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio28_edge_high(&self) -> GPIO28_EDGE_HIGH_R {
        GPIO28_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio28_edge_low(&self) -> GPIO28_EDGE_LOW_R {
        GPIO28_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio28_level_high(&self) -> GPIO28_LEVEL_HIGH_R {
        GPIO28_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio28_level_low(&self) -> GPIO28_LEVEL_LOW_R {
        GPIO28_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio27_edge_high(&self) -> GPIO27_EDGE_HIGH_R {
        GPIO27_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio27_edge_low(&self) -> GPIO27_EDGE_LOW_R {
        GPIO27_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio27_level_high(&self) -> GPIO27_LEVEL_HIGH_R {
        GPIO27_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio27_level_low(&self) -> GPIO27_LEVEL_LOW_R {
        GPIO27_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio26_edge_high(&self) -> GPIO26_EDGE_HIGH_R {
        GPIO26_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio26_edge_low(&self) -> GPIO26_EDGE_LOW_R {
        GPIO26_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio26_level_high(&self) -> GPIO26_LEVEL_HIGH_R {
        GPIO26_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio26_level_low(&self) -> GPIO26_LEVEL_LOW_R {
        GPIO26_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio25_edge_high(&self) -> GPIO25_EDGE_HIGH_R {
        GPIO25_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio25_edge_low(&self) -> GPIO25_EDGE_LOW_R {
        GPIO25_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio25_level_high(&self) -> GPIO25_LEVEL_HIGH_R {
        GPIO25_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio25_level_low(&self) -> GPIO25_LEVEL_LOW_R {
        GPIO25_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio24_edge_high(&self) -> GPIO24_EDGE_HIGH_R {
        GPIO24_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio24_edge_low(&self) -> GPIO24_EDGE_LOW_R {
        GPIO24_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio24_level_high(&self) -> GPIO24_LEVEL_HIGH_R {
        GPIO24_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio24_level_low(&self) -> GPIO24_LEVEL_LOW_R {
        GPIO24_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
