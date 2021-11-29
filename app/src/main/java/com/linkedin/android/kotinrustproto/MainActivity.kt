package com.linkedin.android.kotinrustproto

import android.content.Context
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.linkedin.android.kotinrustproto.databinding.ActivityMainBinding
import com.linkedin.android.proto.Proto
import com.linkedin.android.rsdroid.RustCore
import com.tencent.mmkv.MMKV
import java.lang.RuntimeException

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater);
        binding.text.text = RustCore.instance.greeting();

        MMKV.initialize(this);
        val funEmpty = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.empty();
            }

            override fun String(): String {
                return "Empty"
            }
        }

        RustCore.navHelper.test(
            Proto.TestIn.newBuilder().setA("aa").setB("bb").build(),
            object : RustCore.Callback<Proto.TestOut> {
                override fun onSuccess(arg: Proto.TestOut) {
                    if (arg.a != "test") {
                        throw RuntimeException("ooops in onSuccess")
                    }
                }

                override fun onErr(code: Int, msg: String) {
                    throw RuntimeException("ooops in onErr")
                }
            }
        );

        var map : HashMap<String, String> = HashMap();
        val funMem = object : Fun {
            override fun onCall(i : Int) {
                map.put("test_%d".format(i), "value_%d_10086".format(i));
            }

            override fun String(): String {
                return "Java Mem Write"
            }
        }

        var kv = MMKV.defaultMMKV();
        var funMMKVRead = object : Fun {
            override fun onCall(i : Int) {
                kv.decodeString("test_%d".format(i))
            }

            override fun String(): String {
                return "MMKV Read"
            }
        }

        var funMMKVWrite = object : Fun {
            override fun onCall(i : Int) {
                kv.encode("test_%d".format(i), "value_%d_10086".format(i));
            }

            override fun String(): String {
                return "MMKV Write"
            }
        }

        val funMemRead = object : Fun {
            override fun onCall(i : Int) {
                map.get("test_%d".format(i));
            }

            override fun String(): String {
                return "Java Mem Read"
            }
        }

        val funNativeMem = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.save(
                    "test_%d".format(i),
                    "value_%d_10086".format(i)
                )
            }

            override fun String(): String {
                return "Native Mem Write"
            }
        }

        val funNativeMemRead = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.get(
                    "test_%d".format(i),
                )
            }

            override fun String(): String {
                return "Native Mem Read"
            }
        }

        val funNativeMemProtoRead = object : NavReadFun() {
            override fun String(): String {
                return "Native Mem Read with Proto encoding"
            }
        }

        binding.init.setOnClickListener({
            binding.text.text = testFunc(funEmpty);
            map.clear();
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funMem);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funMemRead);
            RustCore.navHelper.create(
                Proto.OpenIn.newBuilder().setPath("").setMode(0).build(),
                respCallback,
            )
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funNativeMem)
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funNativeMemRead)
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funNativeMemProtoRead)

            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funMMKVRead)
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funMMKVWrite)
        });

        val sharedPref = getPreferences(Context.MODE_PRIVATE)
        val funShareWrite = object : Fun {
            override fun onCall(i : Int) {
                sharedPref.edit().putString("test_%d".format(i), "value_%d_10086".format(i)).commit()
            }

            override fun String(): String {
                return "SharePrefenceWrite"
            }

        }

        val funShareRead = object : Fun {
            override fun onCall(i : Int) {
                sharedPref.getString("test_%d".format(i), "")
            }

            override fun String(): String {
                return "SharePrefenceRead"
            }
        }
        binding.button.setOnClickListener({
            val sharedPref = getPreferences(Context.MODE_PRIVATE)
            sharedPref.edit().clear().commit();
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funShareWrite);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funShareRead);
        })

        val funSledSave = object : Fun {
            override fun onCall(i: Int) {
                RustCore.instance.sledSave(
                    "test_%d".format(i),
                    "value_%d_10086".format(i)
                );
            }

            override fun String(): String {
                return "SledSave without protobuf"
            }
        }


        val funSledWrite = object : NavSaveFun() {
            override fun String(): String {
                return "SledWrite"
            }

        }

        val funSledRead = object : NavReadFun() {
            override fun String(): String {
                return "SledRead"
            }
        }


        binding.fuckingSlow.setOnClickListener({
            val path: String = applicationContext.cacheDir.absolutePath + "/test"
            var dbPath = Proto.OpenIn.newBuilder().setPath(path).setMode(2).build();
            RustCore.navHelper.create(dbPath, respCallback)
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funSledSave);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funSledWrite);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funSledRead);
        });

        val funLmdbWrite = object : NavSaveFun() {
            override fun String(): String {
                return "LmdbWrite"
            }

        }

        val funLmdbRead = object : NavReadFun() {
            override fun String(): String {
                return "LmdbRead"
            }
        }


        binding.lmdb.setOnClickListener {
            val path: String = applicationContext.cacheDir.absolutePath + "/lmdb"
            var dbPath = Proto.OpenIn.newBuilder().setPath(path).setMode(1).build();
            RustCore.navHelper.create(dbPath, respCallback)
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funLmdbWrite);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funLmdbRead);
        };



        val funEmptyGetString = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.greeting()
            }

            override fun String(): String {
                return "Empty get(String)"
            }
        }

        val funEmptyPutString = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.emptySet("Hello world Rust")
            }

            override fun String(): String {
                return "Empty put(String)"
            }
        }

        val funReadString = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.readString("Hello world")
            }

            override fun String(): String {
                return "readString(String)"
            }
        }



        binding.button2.setOnClickListener({
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funEmptyPutString);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funEmptyGetString);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funReadString);
        })

        var br = "Hello 11111111111111111111".toByteArray();
        val funEmptyPutByteArray = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.emptySetB(br)
            }

            override fun String(): String {
                return "Empty put(ByteArray)"
            }
        }

        val funEmptyByteArray = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.byteArray()
            }

            override fun String(): String {
                return "get() :ByteArray"
            }
        }

        val funCopyByteArray = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.copyByteArray(br)
            }

            override fun String(): String {
                return "copyByteArray(ByteArray)"
            }
        }

        val funReadByteArray = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.readByteArray(br)
            }

            override fun String(): String {
                return "readByteArray(ByteArray)"
            }
        }

        val funWriteByteArray = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.writeByteArray(br);
            }

            override fun String(): String {
                return "writeByteArray(ByteArray)"
            }
        }

        binding.button3.setOnClickListener({
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funEmptyPutByteArray);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funEmptyByteArray);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funCopyByteArray);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funReadByteArray);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funWriteByteArray);
        })


        var bb = java.nio.ByteBuffer.allocateDirect(10);
        val funReadByteBuffer = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.readByteBuffer(bb);
            }

            override fun String(): String {
                return "readByteBuffer(ByteBuffer)"
            }
        }

        val funWriteByteBuffer = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.writeByteBuffer(bb);
            }

            override fun String(): String {
                return "writeByteBuffer(ByteBuffer)"
            }
        }
        binding.button4.setOnClickListener({
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funReadByteBuffer);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funWriteByteBuffer);
        })



        setContentView(binding.root);

    }

    fun testFunc(ff : Fun) : String {
        val iterCount = 10000;
        var start = System.currentTimeMillis();
        for (i in 0..iterCount) {
            ff.onCall(i);
        }
        var end = System.currentTimeMillis();

        return "%s takes %d ms ".format(ff.String(), end - start);
    }


    abstract class NavSaveFun : Fun {
        override fun onCall(i: Int) {
            RustCore.navHelper.save(Proto.SaveIn.newBuilder()
                .setKey("test_%d".format(i))
                .setVal("value_%d_10086".format(i)).build(),
                respCallback)
        }

        abstract override fun String() : String

    }

    abstract class NavReadFun : Fun {
        override fun onCall(i: Int) {
            RustCore.navHelper.get(Proto.Str.newBuilder().setVal("test_%d".format(i)).build()
                , strCallback)
        }

        abstract override fun String(): String;

    }

    interface Fun {
        fun onCall(i : Int)
        fun String() : String
    }
}

object respCallback : RustCore.Callback<Proto.Resp> {
    override fun onSuccess(arg: Proto.Resp) {
        super.onSuccess(arg)
    }

    override fun onErr(code: Int, msg: String) {
        super.onErr(code, msg)
    }
}

object strCallback : RustCore.Callback<Proto.Str> {
    override fun onSuccess(arg: Proto.Str) {
        super.onSuccess(arg)
    }

    override fun onErr(code: Int, msg: String) {
        throw RuntimeException("ooops")
    }
}
