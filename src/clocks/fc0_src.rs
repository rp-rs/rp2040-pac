#[doc = "Register `FC0_SRC` reader"]
pub struct R(crate::R<FC0_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC0_SRC` writer"]
pub struct W(crate::W<FC0_SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC0_SRC_SPEC>;
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
impl From<crate::W<FC0_SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC0_SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FC0_SRC_A {
    #[doc = "0: `0`"]
    NULL = 0,
    #[doc = "1: `1`"]
    PLL_SYS_CLKSRC_PRIMARY = 1,
    #[doc = "2: `10`"]
    PLL_USB_CLKSRC_PRIMARY = 2,
    #[doc = "3: `11`"]
    ROSC_CLKSRC = 3,
    #[doc = "4: `100`"]
    ROSC_CLKSRC_PH = 4,
    #[doc = "5: `101`"]
    XOSC_CLKSRC = 5,
    #[doc = "6: `110`"]
    CLKSRC_GPIN0 = 6,
    #[doc = "7: `111`"]
    CLKSRC_GPIN1 = 7,
    #[doc = "8: `1000`"]
    CLK_REF = 8,
    #[doc = "9: `1001`"]
    CLK_SYS = 9,
    #[doc = "10: `1010`"]
    CLK_PERI = 10,
    #[doc = "11: `1011`"]
    CLK_USB = 11,
    #[doc = "12: `1100`"]
    CLK_ADC = 12,
    #[doc = "13: `1101`"]
    CLK_RTC = 13,
}
impl From<FC0_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FC0_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FC0_SRC` reader - "]
pub struct FC0_SRC_R(crate::FieldReader<u8, FC0_SRC_A>);
impl FC0_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FC0_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FC0_SRC_A> {
        match self.bits {
            0 => Some(FC0_SRC_A::NULL),
            1 => Some(FC0_SRC_A::PLL_SYS_CLKSRC_PRIMARY),
            2 => Some(FC0_SRC_A::PLL_USB_CLKSRC_PRIMARY),
            3 => Some(FC0_SRC_A::ROSC_CLKSRC),
            4 => Some(FC0_SRC_A::ROSC_CLKSRC_PH),
            5 => Some(FC0_SRC_A::XOSC_CLKSRC),
            6 => Some(FC0_SRC_A::CLKSRC_GPIN0),
            7 => Some(FC0_SRC_A::CLKSRC_GPIN1),
            8 => Some(FC0_SRC_A::CLK_REF),
            9 => Some(FC0_SRC_A::CLK_SYS),
            10 => Some(FC0_SRC_A::CLK_PERI),
            11 => Some(FC0_SRC_A::CLK_USB),
            12 => Some(FC0_SRC_A::CLK_ADC),
            13 => Some(FC0_SRC_A::CLK_RTC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NULL`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        **self == FC0_SRC_A::NULL
    }
    #[doc = "Checks if the value of the field is `PLL_SYS_CLKSRC_PRIMARY`"]
    #[inline(always)]
    pub fn is_pll_sys_clksrc_primary(&self) -> bool {
        **self == FC0_SRC_A::PLL_SYS_CLKSRC_PRIMARY
    }
    #[doc = "Checks if the value of the field is `PLL_USB_CLKSRC_PRIMARY`"]
    #[inline(always)]
    pub fn is_pll_usb_clksrc_primary(&self) -> bool {
        **self == FC0_SRC_A::PLL_USB_CLKSRC_PRIMARY
    }
    #[doc = "Checks if the value of the field is `ROSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_rosc_clksrc(&self) -> bool {
        **self == FC0_SRC_A::ROSC_CLKSRC
    }
    #[doc = "Checks if the value of the field is `ROSC_CLKSRC_PH`"]
    #[inline(always)]
    pub fn is_rosc_clksrc_ph(&self) -> bool {
        **self == FC0_SRC_A::ROSC_CLKSRC_PH
    }
    #[doc = "Checks if the value of the field is `XOSC_CLKSRC`"]
    #[inline(always)]
    pub fn is_xosc_clksrc(&self) -> bool {
        **self == FC0_SRC_A::XOSC_CLKSRC
    }
    #[doc = "Checks if the value of the field is `CLKSRC_GPIN0`"]
    #[inline(always)]
    pub fn is_clksrc_gpin0(&self) -> bool {
        **self == FC0_SRC_A::CLKSRC_GPIN0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_GPIN1`"]
    #[inline(always)]
    pub fn is_clksrc_gpin1(&self) -> bool {
        **self == FC0_SRC_A::CLKSRC_GPIN1
    }
    #[doc = "Checks if the value of the field is `CLK_REF`"]
    #[inline(always)]
    pub fn is_clk_ref(&self) -> bool {
        **self == FC0_SRC_A::CLK_REF
    }
    #[doc = "Checks if the value of the field is `CLK_SYS`"]
    #[inline(always)]
    pub fn is_clk_sys(&self) -> bool {
        **self == FC0_SRC_A::CLK_SYS
    }
    #[doc = "Checks if the value of the field is `CLK_PERI`"]
    #[inline(always)]
    pub fn is_clk_peri(&self) -> bool {
        **self == FC0_SRC_A::CLK_PERI
    }
    #[doc = "Checks if the value of the field is `CLK_USB`"]
    #[inline(always)]
    pub fn is_clk_usb(&self) -> bool {
        **self == FC0_SRC_A::CLK_USB
    }
    #[doc = "Checks if the value of the field is `CLK_ADC`"]
    #[inline(always)]
    pub fn is_clk_adc(&self) -> bool {
        **self == FC0_SRC_A::CLK_ADC
    }
    #[doc = "Checks if the value of the field is `CLK_RTC`"]
    #[inline(always)]
    pub fn is_clk_rtc(&self) -> bool {
        **self == FC0_SRC_A::CLK_RTC
    }
}
impl core::ops::Deref for FC0_SRC_R {
    type Target = crate::FieldReader<u8, FC0_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0_SRC` writer - "]
pub struct FC0_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC0_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn null(self) -> &'a mut W {
        self.variant(FC0_SRC_A::NULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_sys_clksrc_primary(self) -> &'a mut W {
        self.variant(FC0_SRC_A::PLL_SYS_CLKSRC_PRIMARY)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_usb_clksrc_primary(self) -> &'a mut W {
        self.variant(FC0_SRC_A::PLL_USB_CLKSRC_PRIMARY)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rosc_clksrc(self) -> &'a mut W {
        self.variant(FC0_SRC_A::ROSC_CLKSRC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rosc_clksrc_ph(self) -> &'a mut W {
        self.variant(FC0_SRC_A::ROSC_CLKSRC_PH)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn xosc_clksrc(self) -> &'a mut W {
        self.variant(FC0_SRC_A::XOSC_CLKSRC)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clksrc_gpin0(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLKSRC_GPIN0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn clksrc_gpin1(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLKSRC_GPIN1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn clk_ref(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLK_REF)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn clk_sys(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLK_SYS)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn clk_peri(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLK_PERI)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn clk_usb(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLK_USB)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn clk_adc(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLK_ADC)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn clk_rtc(self) -> &'a mut W {
        self.variant(FC0_SRC_A::CLK_RTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fc0_src(&self) -> FC0_SRC_R {
        FC0_SRC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fc0_src(&mut self) -> FC0_SRC_W {
        FC0_SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock sent to frequency counter, set to 0 when not required  
 Writing to this register initiates the frequency count  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_src](index.html) module"]
pub struct FC0_SRC_SPEC;
impl crate::RegisterSpec for FC0_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_src::R](R) reader structure"]
impl crate::Readable for FC0_SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc0_src::W](W) writer structure"]
impl crate::Writable for FC0_SRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FC0_SRC to value 0"]
impl crate::Resettable for FC0_SRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
