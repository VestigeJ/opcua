// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EventNotificationList {
    pub events: Option<Vec<EventFieldList>>,
}

impl BinaryEncoder<EventNotificationList> for EventNotificationList {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.events);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.events)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let events: Option<Vec<EventFieldList>> = read_array(stream)?;
        Ok(EventNotificationList {
            events: events,
        })
    }
}