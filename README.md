```
   Compiling nacro v0.1.0 (/tmp/nacro)
error: expected identifier, found `OutboundDisconnected`
  --> src/main.rs:5:40
   |
5  |                   let event = DnetEvent::$event_name(dnet::$event_name $($code)*);
   |                                          ^^^^^^^^^^^ expected identifier
...
24 | /     dnet2k!(foo, OutboundDisconnected, {
25 | |         slot,
26 | |         err: e.to_string()
27 | |     });
   | |______- in this macro invocation
   |
   = note: this error originates in the macro `dnet2k` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not compile `nacro` (bin "nacro") due to previous error
```
