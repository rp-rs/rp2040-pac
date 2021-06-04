#[doc = "Reader of register SM_PINCTRL"]
pub type R = crate::R<u32, super::SM_PINCTRL>;
#[doc = "Writer for register SM_PINCTRL"]
pub type W = crate::W<u32, super::SM_PINCTRL>;
#[doc = "Register SM_PINCTRL `reset()`'s with value 0x1400_0000"]
impl crate::ResetValue for super::SM_PINCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1400_0000
    }
}
#[doc = "Reader of field `SIDESET_COUNT`"]
pub type SIDESET_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIDESET_COUNT`"]
pub struct SIDESET_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDESET_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `SET_COUNT`"]
pub type SET_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SET_COUNT`"]
pub struct SET_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Reader of field `OUT_COUNT`"]
pub type OUT_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_COUNT`"]
pub struct OUT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `IN_BASE`"]
pub type IN_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN_BASE`"]
pub struct IN_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `SIDESET_BASE`"]
pub type SIDESET_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIDESET_BASE`"]
pub struct SIDESET_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDESET_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SET_BASE`"]
pub type SET_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SET_BASE`"]
pub struct SET_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `OUT_BASE`"]
pub type OUT_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_BASE`"]
pub struct OUT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
    #[inline(always)]
    pub fn sideset_count(&self) -> SIDESET_COUNT_R {
        SIDESET_COUNT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
    #[inline(always)]
    pub fn set_count(&self) -> SET_COUNT_R {
        SET_COUNT_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
    #[inline(always)]
    pub fn out_count(&self) -> OUT_COUNT_R {
        OUT_COUNT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 15:19 - The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
    #[inline(always)]
    pub fn in_base(&self) -> IN_BASE_R {
        IN_BASE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
    #[inline(always)]
    pub fn sideset_base(&self) -> SIDESET_BASE_R {
        SIDESET_BASE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
    #[inline(always)]
    pub fn set_base(&self) -> SET_BASE_R {
        SET_BASE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
    #[inline(always)]
    pub fn out_base(&self) -> OUT_BASE_R {
        OUT_BASE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
    #[inline(always)]
    pub fn sideset_count(&mut self) -> SIDESET_COUNT_W {
        SIDESET_COUNT_W { w: self }
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
    #[inline(always)]
    pub fn set_count(&mut self) -> SET_COUNT_W {
        SET_COUNT_W { w: self }
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
    #[inline(always)]
    pub fn out_count(&mut self) -> OUT_COUNT_W {
        OUT_COUNT_W { w: self }
    }
    #[doc = "Bits 15:19 - The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
    #[inline(always)]
    pub fn in_base(&mut self) -> IN_BASE_W {
        IN_BASE_W { w: self }
    }
    #[doc = "Bits 10:14 - The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
    #[inline(always)]
    pub fn sideset_base(&mut self) -> SIDESET_BASE_W {
        SIDESET_BASE_W { w: self }
    }
    #[doc = "Bits 5:9 - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
    #[inline(always)]
    pub fn set_base(&mut self) -> SET_BASE_W {
        SET_BASE_W { w: self }
    }
    #[doc = "Bits 0:4 - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
    #[inline(always)]
    pub fn out_base(&mut self) -> OUT_BASE_W {
        OUT_BASE_W { w: self }
    }
}
