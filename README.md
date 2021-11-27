# KotlinRustProto
a prototype project integrating jni rust into Kotlin and using protobuf to make them work together


# How to start
add a RPC call in DroidBackendService

```
service DroidBackendService {
    // add a new rpc call here
    rpc Test(TestIn) returns (TestOut);
    
}

message TestIn {
  string A = 1;
  string B = 2;
}

message TestOut {
  string A = 1;
}

```

# Do the small implementation work

1. try building and wait, or go get a coffee to get the compiling failure
2. implement the rust code

```
// import new type
use crate::proto::TestOut;

impl DroidBackendService for Backend {

// add new implementation
fn test(&self, input: TestIn) -> BackendResult<TestOut> {
        Ok(TestOut {
            a : "test".into(),
        })
    }
  
```
3. build again, and try it out in Kotin

```
RustCore.navHelper.test(
            Proto.TestIn.newBuilder().setA("aa").setB("bb").build(),
            object : RustCore.Callback<Proto.TestOut> {
                override fun onSuccess(arg: Proto.TestOut) {
                    // since we've set it as "test" 
                    if (arg.a != "test") {
                        throw RuntimeException("ooops in onSuccess")
                    }
                }

                override fun onErr(code: Int, msg: String) {
                    throw RuntimeException("ooops in onErr")
                }
            }
        );
```

# What's more
My first goal is to embed a high performance kvstore into Kotlin, namely lmdb, sled, etc.
Then it goes to what it likes now after some iterations and could do some little more things. [here](https://blog.gaxxx.me/kotin-with-rust/) is how I make it work step by step...

Also as it works in this demo
run each function for 1000 times, and here is the result, in my real phone (Xiaomi 10).


|  Function | time used |
|----|----|
| empty jni call | 0ms | 
| Java Hashmap set | 30ms | 
| Java Hashmap get | 10ms | 
| Jni cocurrent Hashmap set | 130ms | 
| Jni cocurrent Hashmap get | 106ms | 
| SharedPrefence write | 932ms | 
| SharedPrefence read | 9ms |
| Sled write | 227ms |
| Sled read | 152ms|
| Lmdb write | 459ms |
| Lmdb read | 133ms|

Sled outperforms SharedPrefence & Lmdb, a lot. Of course, rust is stable enough, but to put it into real productions, there is more work to do, something like

1. add multiprocess support
2. space error handling

But on the other hand, with protobuf support, we could build some core features in Rust and invoke them in other languages like swift, flutter etc.

happy hacking
