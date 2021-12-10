#[doc = "Register `CLK_USB_DIV` reader"]
pub struct R(crate::R<CLK_USB_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_USB_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_USB_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_USB_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_USB_DIV` writer"]
pub struct W(crate::W<CLK_USB_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_USB_DIV_SPEC>;
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
impl From<crate::W<CLK_USB_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_USB_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Integer component of the divisor, 0 -> divide by 2^16"]
pub struct INT_R(crate::FieldReader<u8, u8>);
impl INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Integer component of the divisor, 0 -> divide by 2^16"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divisor, can be changed on-the-fly  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_usb_div](index.html) module"]
pub struct CLK_USB_DIV_SPEC;
impl crate::RegisterSpec for CLK_USB_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_usb_div::R](R) reader structure"]
impl crate::Readable for CLK_USB_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_usb_div::W](W) writer structure"]
impl crate::Writable for CLK_USB_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_USB_DIV to value 0x0100"]
impl crate::Resettable for CLK_USB_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
