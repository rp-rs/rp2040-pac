#[doc = "Register `BUS_PRIORITY_ACK` reader"]
pub struct R(crate::R<BUS_PRIORITY_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_PRIORITY_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_PRIORITY_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_PRIORITY_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUS_PRIORITY_ACK` reader - Goes to 1 once all arbiters have registered the new global priority levels.  
 Arbiters update their local priority when servicing a new nonsequential access.  
 In normal circumstances this will happen almost immediately."]
pub struct BUS_PRIORITY_ACK_R(crate::FieldReader<bool, bool>);
impl BUS_PRIORITY_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_PRIORITY_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_PRIORITY_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Goes to 1 once all arbiters have registered the new global priority levels.  
 Arbiters update their local priority when servicing a new nonsequential access.  
 In normal circumstances this will happen almost immediately."]
    #[inline(always)]
    pub fn bus_priority_ack(&self) -> BUS_PRIORITY_ACK_R {
        BUS_PRIORITY_ACK_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Bus priority acknowledge  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [bus_priority_ack](index.html) module"]
pub struct BUS_PRIORITY_ACK_SPEC;
impl crate::RegisterSpec for BUS_PRIORITY_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_priority_ack::R](R) reader structure"]
impl crate::Readable for BUS_PRIORITY_ACK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS_PRIORITY_ACK to value 0"]
impl crate::Resettable for BUS_PRIORITY_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
