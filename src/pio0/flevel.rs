#[doc = "Reader of register FLEVEL"]
pub type R = crate::R<u32, super::FLEVEL>;
#[doc = "Reader of field `RX3`"]
pub type RX3_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX3`"]
pub type TX3_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX2`"]
pub type RX2_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX2`"]
pub type TX2_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX1`"]
pub type RX1_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX1`"]
pub type TX1_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX0`"]
pub type RX0_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX0`"]
pub type TX0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx3(&self) -> RX3_R {
        RX3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn tx3(&self) -> TX3_R {
        TX3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rx2(&self) -> RX2_R {
        RX2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn tx2(&self) -> TX2_R {
        TX2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rx1(&self) -> RX1_R {
        RX1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tx1(&self) -> TX1_R {
        TX1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rx0(&self) -> RX0_R {
        RX0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tx0(&self) -> TX0_R {
        TX0_R::new((self.bits & 0x0f) as u8)
    }
}
