#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC.  
 If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ENABLE_A {
    #[doc = "3358: `110100011110`"]
    DISABLE = 3358,
    #[doc = "4011: `111110101011`"]
    ENABLE = 4011,
}
impl From<ENABLE_A> for u16 {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENABLE` reader - On power-up this field is initialised to DISABLE and the chip runs from the ROSC.  
 If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
pub struct ENABLE_R(crate::FieldReader<u16, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_A> {
        match self.bits {
            3358 => Some(ENABLE_A::DISABLE),
            4011 => Some(ENABLE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<u16, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - On power-up this field is initialised to DISABLE and the chip runs from the ROSC.  
 If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`110100011110`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "`111110101011`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | ((value as u32 & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Frequency range. This resets to 0xAA0 and cannot be changed.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FREQ_RANGE_A {
    #[doc = "2720: `101010100000`"]
    _1_15MHZ = 2720,
    #[doc = "2721: `101010100001`"]
    RESERVED_1 = 2721,
    #[doc = "2722: `101010100010`"]
    RESERVED_2 = 2722,
    #[doc = "2723: `101010100011`"]
    RESERVED_3 = 2723,
}
impl From<FREQ_RANGE_A> for u16 {
    #[inline(always)]
    fn from(variant: FREQ_RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQ_RANGE` reader - Frequency range. This resets to 0xAA0 and cannot be changed."]
pub struct FREQ_RANGE_R(crate::FieldReader<u16, FREQ_RANGE_A>);
impl FREQ_RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FREQ_RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_RANGE_A> {
        match self.bits {
            2720 => Some(FREQ_RANGE_A::_1_15MHZ),
            2721 => Some(FREQ_RANGE_A::RESERVED_1),
            2722 => Some(FREQ_RANGE_A::RESERVED_2),
            2723 => Some(FREQ_RANGE_A::RESERVED_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_15MHZ`"]
    #[inline(always)]
    pub fn is_1_15mhz(&self) -> bool {
        **self == FREQ_RANGE_A::_1_15MHZ
    }
    #[doc = "Checks if the value of the field is `RESERVED_1`"]
    #[inline(always)]
    pub fn is_reserved_1(&self) -> bool {
        **self == FREQ_RANGE_A::RESERVED_1
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        **self == FREQ_RANGE_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        **self == FREQ_RANGE_A::RESERVED_3
    }
}
impl core::ops::Deref for FREQ_RANGE_R {
    type Target = crate::FieldReader<u16, FREQ_RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ_RANGE` writer - Frequency range. This resets to 0xAA0 and cannot be changed."]
pub struct FREQ_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`101010100000`"]
    #[inline(always)]
    pub fn _1_15mhz(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::_1_15MHZ)
    }
    #[doc = "`101010100001`"]
    #[inline(always)]
    pub fn reserved_1(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::RESERVED_1)
    }
    #[doc = "`101010100010`"]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::RESERVED_2)
    }
    #[doc = "`101010100011`"]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::RESERVED_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:23 - On power-up this field is initialised to DISABLE and the chip runs from the ROSC.  
 If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Frequency range. This resets to 0xAA0 and cannot be changed."]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:23 - On power-up this field is initialised to DISABLE and the chip runs from the ROSC.  
 If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.  
 The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:11 - Frequency range. This resets to 0xAA0 and cannot be changed."]
    #[inline(always)]
    pub fn freq_range(&mut self) -> FREQ_RANGE_W {
        FREQ_RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal Oscillator Control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
