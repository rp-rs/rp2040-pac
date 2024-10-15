#[doc = "Register `BUS_PRIORITY_ACK` reader"]
pub type R = crate::R<BUS_PRIORITY_ACK_SPEC>;
#[doc = "Field `BUS_PRIORITY_ACK` reader - Goes to 1 once all arbiters have registered the new global priority levels.  
 Arbiters update their local priority when servicing a new nonsequential access.  
 In normal circumstances this will happen almost immediately."]
pub type BUS_PRIORITY_ACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Goes to 1 once all arbiters have registered the new global priority levels.  
 Arbiters update their local priority when servicing a new nonsequential access.  
 In normal circumstances this will happen almost immediately."]
    #[inline(always)]
    pub fn bus_priority_ack(&self) -> BUS_PRIORITY_ACK_R {
        BUS_PRIORITY_ACK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Bus priority acknowledge  

You can [`read`](crate::Reg::read) this register and get [`bus_priority_ack::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_PRIORITY_ACK_SPEC;
impl crate::RegisterSpec for BUS_PRIORITY_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_priority_ack::R`](R) reader structure"]
impl crate::Readable for BUS_PRIORITY_ACK_SPEC {}
#[doc = "`reset()` method sets BUS_PRIORITY_ACK to value 0"]
impl crate::Resettable for BUS_PRIORITY_ACK_SPEC {
    const RESET_VALUE: u32 = 0;
}
