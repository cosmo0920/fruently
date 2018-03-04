var searchIndex = {};
searchIndex["fruently"] = {"doc":"`fruently` is a yet another Fluentd logger for Rust.","items":[[0,"fluent","fruently","Send record(s) into Fluentd.",null,null],[3,"Fluent","fruently::fluent","",null,null],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"fluent"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"fluent"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"fluent"}],"output":{"name":"bool"}}],[11,"new","","Create Fluent type.",0,{"inputs":[{"name":"a"},{"name":"t"}],"output":{"name":"fluent"}}],[11,"new_with_conf","","",0,{"inputs":[{"name":"a"},{"name":"t"},{"name":"retryconf"}],"output":{"name":"fluent"}}],[0,"record","fruently","Implement record manupulation mechanisms.",null,null],[3,"Record","fruently::record","",null,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"record"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"record"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"self"},{"name":"record"}],"output":{"name":"bool"}}],[11,"new","","",1,{"inputs":[{"name":"string"},{"name":"tm"},{"name":"t"}],"output":{"name":"record"}}],[11,"dump","","",1,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"serialize","","",1,{"inputs":[{"name":"self"},{"name":"s"}],"output":{"name":"result"}}],[0,"forwardable","fruently","Implement concrete sending record(s) specifications.",null,null],[0,"json","fruently::forwardable","Send record as forwardable json.",null,null],[11,"post","fruently::fluent","Post record into Fluentd. Without time version.",0,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["fluenterror"],"name":"result"}}],[11,"post_with_time","","Post record into Fluentd. With time version.",0,{"inputs":[{"name":"self"},{"name":"t"},{"name":"tm"}],"output":{"generics":["fluenterror"],"name":"result"}}],[0,"msgpack","fruently::forwardable","Send record as msgpack.",null,null],[11,"post","fruently::fluent","Post record into Fluentd. Without time version.",0,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["fluenterror"],"name":"result"}}],[11,"post_with_time","","Post record into Fluentd. With time version.",0,{"inputs":[{"name":"self"},{"name":"t"},{"name":"tm"}],"output":{"generics":["fluenterror"],"name":"result"}}],[0,"forward","fruently::forwardable","Send records as forward mode.",null,null],[3,"Forward","fruently::forwardable::forward","",null,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"forward"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"forward"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"forward"}],"output":{"name":"bool"}}],[11,"new","","",2,{"inputs":[{"name":"string"},{"generics":["entry"],"name":"vec"}],"output":{"name":"forward"}}],[11,"dump","","",2,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"post","fruently::fluent","Post `Vec<Entry<T>>` into Fluentd.",0,{"inputs":[{"name":"self"},{"generics":["entry"],"name":"vec"}],"output":{"generics":["fluenterror"],"name":"result"}}],[6,"Entry","fruently::forwardable","",null,null],[8,"JsonForwardable","","",null,null],[10,"post","","",3,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["fluenterror"],"name":"result"}}],[10,"post_with_time","","",3,{"inputs":[{"name":"self"},{"name":"t"},{"name":"tm"}],"output":{"generics":["fluenterror"],"name":"result"}}],[8,"MsgpackForwardable","","",null,null],[10,"post","","",4,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["fluenterror"],"name":"result"}}],[10,"post_with_time","","",4,{"inputs":[{"name":"self"},{"name":"t"},{"name":"tm"}],"output":{"generics":["fluenterror"],"name":"result"}}],[8,"Forwardable","","",null,null],[10,"post","","",5,{"inputs":[{"name":"self"},{"generics":["entry"],"name":"vec"}],"output":{"generics":["fluenterror"],"name":"result"}}],[0,"retry_conf","fruently","Retry sending records configuration.",null,null],[3,"RetryConf","fruently::retry_conf","You can calculate retrying interval as the following equation:",null,null],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"retryconf"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"retryconf"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"retryconf"}],"output":{"name":"bool"}}],[11,"default","","",6,{"inputs":[],"output":{"name":"retryconf"}}],[11,"new","","",6,{"inputs":[],"output":{"name":"retryconf"}}],[11,"max","","",6,{"inputs":[{"name":"self"},{"name":"u64"}],"output":{"name":"retryconf"}}],[11,"multiplier","","",6,{"inputs":[{"name":"self"},{"name":"f64"}],"output":{"name":"retryconf"}}],[11,"store_file","","",6,{"inputs":[{"name":"self"},{"name":"pathbuf"}],"output":{"name":"retryconf"}}],[11,"need_to_store","","",6,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"store_path","","",6,{"inputs":[{"name":"self"}],"output":{"generics":["pathbuf"],"name":"option"}}],[11,"build","","",6,null],[0,"store_buffer","fruently","Store buffer when failing to send events.",null,null],[5,"maybe_write_events","fruently::store_buffer","Write events buffer into file with TSV format.",null,{"inputs":[{"name":"retryconf"},{"name":"t"},{"name":"fluenterror"}],"output":{"generics":["fluenterror"],"name":"result"}}],[0,"error","fruently","Implement error types for fruently crate.",null,null],[4,"FluentError","fruently::error","",null,null],[13,"JsonEncode","","",7,null],[13,"MsgpackEncode","","",7,null],[13,"IO","","",7,null],[13,"Retry","","",7,null],[13,"FileStored","","",7,null],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"generics":["error"],"name":"result"}}],[11,"description","","",7,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"from","","",7,{"inputs":[{"name":"error"}],"output":{"name":"fluenterror"}}],[11,"from","","",7,{"inputs":[{"name":"error"}],"output":{"name":"fluenterror"}}],[11,"from","","",7,{"inputs":[{"name":"retryerror"}],"output":{"name":"fluenterror"}}],[11,"from","","",7,{"inputs":[{"name":"error"}],"output":{"name":"fluenterror"}}],[0,"event_record","fruently","Implement EventTime record manupulation mechanisms.",null,null],[3,"EventRecord","fruently::event_record","",null,null],[3,"Event","","",null,null],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",8,{"inputs":[{"name":"self"}],"output":{"name":"eventrecord"}}],[11,"eq","","",8,{"inputs":[{"name":"self"},{"name":"eventrecord"}],"output":{"name":"bool"}}],[11,"ne","","",8,{"inputs":[{"name":"self"},{"name":"eventrecord"}],"output":{"name":"bool"}}],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",9,{"inputs":[{"name":"self"}],"output":{"name":"event"}}],[11,"eq","","",9,{"inputs":[{"name":"self"},{"name":"event"}],"output":{"name":"bool"}}],[11,"ne","","",9,{"inputs":[{"name":"self"},{"name":"event"}],"output":{"name":"bool"}}],[11,"new","","",9,{"inputs":[{"name":"eventtime"},{"name":"t"}],"output":{"name":"event"}}],[11,"get_record","","",9,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"get_event_time","","",9,{"inputs":[{"name":"self"}],"output":{"name":"eventtime"}}],[11,"new","","",8,{"inputs":[{"name":"string"},{"name":"tm"},{"name":"t"}],"output":{"name":"eventrecord"}}],[11,"dump","","",8,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"serialize","","",8,{"inputs":[{"name":"self"},{"name":"s"}],"output":{"name":"result"}}],[0,"event_time","fruently","Forward proticol v1 version of high accuracy time representation.",null,null],[3,"EventTime","fruently::event_time","",null,null],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",10,{"inputs":[{"name":"self"}],"output":{"name":"eventtime"}}],[11,"eq","","",10,{"inputs":[{"name":"self"},{"name":"eventtime"}],"output":{"name":"bool"}}],[11,"ne","","",10,{"inputs":[{"name":"self"},{"name":"eventtime"}],"output":{"name":"bool"}}],[11,"new","","",10,{"inputs":[{"name":"tm"}],"output":{"name":"eventtime"}}],[11,"get_time","","",10,{"inputs":[{"name":"self"}],"output":{"name":"tm"}}],[11,"serialize","","",10,{"inputs":[{"name":"self"},{"name":"s"}],"output":{"name":"result"}}],[0,"dumpable","fruently","Traits for dump record(s).",null,null],[8,"Dumpable","fruently::dumpable","",null,null],[10,"dump","","",11,{"inputs":[{"name":"self"}],"output":{"name":"string"}}]],"paths":[[3,"Fluent"],[3,"Record"],[3,"Forward"],[8,"JsonForwardable"],[8,"MsgpackForwardable"],[8,"Forwardable"],[3,"RetryConf"],[4,"FluentError"],[3,"EventRecord"],[3,"Event"],[3,"EventTime"],[8,"Dumpable"]]};
initSearch(searchIndex);
