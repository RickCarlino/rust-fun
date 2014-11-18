{"changed":true,"filter":false,"title":"main.rs","tooltip":"/src/main.rs","value":"extern crate redis;\nuse redis::Commands;\n\nmod agent;\n\nfn main() {\n  let x = fetch_an_integer();\n  match x {\n      Ok(v) => {\n          println!(\"working with version: {}\", v);\n      }\n      Err(e) => {\n          println!(\"error parsing header: {}\", e);\n      }\n  };\n}\n\nfn fetch_an_integer() -> redis::RedisResult<int> {\n    // connect to redis\n    let client = try!(redis::Client::open(\"redis://127.0.0.1/\"));\n    let con = try!(client.get_connection());\n    // throw away the result, just make sure it does not fail\n    let _ : () = try!(con.set(\"my_key\", 42i));\n    // read back the key and return it.  Because the return value\n    // from the function is a result for integer this will automatically\n    // convert into one.\n    con.get(\"my_key\")\n}\n\n\n\n\n//objects in rust \n\n\n","undoManager":{"mark":-1,"position":11,"stack":[[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":2,"column":0},"end":{"row":3,"column":0}},"text":"\n"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":0},"end":{"row":4,"column":0}},"text":"\n"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":0},"end":{"row":3,"column":1}},"text":"m"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":1},"end":{"row":3,"column":2}},"text":"o"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":2},"end":{"row":3,"column":3}},"text":"d"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":3},"end":{"row":3,"column":4}},"text":" "}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":4},"end":{"row":3,"column":5}},"text":"a"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":5},"end":{"row":3,"column":6}},"text":"g"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":6},"end":{"row":3,"column":7}},"text":"e"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":7},"end":{"row":3,"column":8}},"text":"n"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":8},"end":{"row":3,"column":9}},"text":"t"}]}],[{"group":"doc","deltas":[{"action":"insertText","range":{"start":{"row":3,"column":9},"end":{"row":3,"column":10}},"text":";"}]}]]},"ace":{"folds":[],"scrolltop":0,"scrollleft":0,"selection":{"start":{"row":34,"column":0},"end":{"row":34,"column":0},"isBackwards":false},"options":{"guessTabSize":true,"useWrapMode":false,"wrapToView":true},"firstLineState":0},"timestamp":1416192428000}