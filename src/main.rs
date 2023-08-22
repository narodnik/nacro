macro_rules! dnet2k {
    ($self:expr, $event_name:expr, $($code:tt)*) => {
        {
            if $self {
                let event = DnetEvent::$event_name(dnet::$event_name $($code)*);
                //$self.p2p().dnet_notify(event).await;
            }
        }
    };
}

pub struct OutboundDisconnected {
    pub slot: u32,
    pub err: String,
}

pub enum DnetEvent {
    OutboundDisconnected(OutboundDisconnected),
}


fn main() {
    let foo = true;
    dnet2k!(foo, OutboundDisconnected, {
        slot,
        err: e.to_string()
    });

    println!("Hello, world!");
}
