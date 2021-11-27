package com.linkedin.android.kotinrustproto

import android.content.Context
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.linkedin.android.kotinrustproto.databinding.ActivityMainBinding
import com.linkedin.android.proto.Native
import com.linkedin.android.rsdroid.RustCore
import java.lang.Exception

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater);
        binding.text.text = RustCore.instance.greeting();

        val funEmpty = object : Fun {
            override fun onCall(i : Int) {
                RustCore.instance.empty();
            }

            override fun String(): String {
                return "Empty"
            }
        }

        var map : HashMap<String, String> = HashMap();
        val funMem = object : Fun {
            override fun onCall(i : Int) {
                map.put("test_%d".format(i), "value_%d_10086".format(i));
            }

            override fun String(): String {
                return "Java Mem Write"
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

        val funNativeMemRead = object : NavReadFun() {
            override fun String(): String {
                return "Native Mem Read"
            }
        }

        binding.init.setOnClickListener({
            binding.text.text = testFunc(funEmpty);
            map.clear();
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funMem);
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funMemRead);
            RustCore.navHelper.create(
                Native.OpenIn.newBuilder().setPath("").setMode(0).build(),
                emptyCallback,
            )
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funNativeMem)
            binding.text.text = binding.text.text.toString() + "\n" + testFunc(funNativeMemRead)
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
            var dbPath = Native.OpenIn.newBuilder().setPath(path).setMode(2).build();
            RustCore.navHelper.create(dbPath, emptyCallback)
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
            var dbPath = Native.OpenIn.newBuilder().setPath(path).setMode(1).build();
            RustCore.navHelper.create(dbPath, emptyCallback)
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
        val iterCount = 1000;
        var start = System.currentTimeMillis();
        for (i in 0..iterCount) {
            ff.onCall(i);
        }
        var end = System.currentTimeMillis();

        return "%s takes %d ms ".format(ff.String(), end - start);
    }


    abstract class NavSaveFun : Fun {
        override fun onCall(i: Int) {
            RustCore.navHelper.save(Native.SaveIn.newBuilder()
                .setKey("test_%d".format(i))
                .setVal("value_%d_10086".format(i)).build(),
                emptyCallback)
        }

        abstract override fun String() : String

    }

    abstract class NavReadFun : Fun {
        override fun onCall(i: Int) {
            RustCore.navHelper.get(Native.Str.newBuilder().setVal("test_%d".format(i)).build()
                , strCallback)
        }

        abstract override fun String(): String;

    }

    interface Fun {
        fun onCall(i : Int)
        fun String() : String
    }
}

object emptyCallback : RustCore.Callback<Native.Empty> {
    override fun onSuccess(arg: Native.Empty) {
        super.onSuccess(arg)
    }

    override fun onErr(code: Int, msg: String) {
        super.onErr(code, msg)
    }
}

object strCallback : RustCore.Callback<Native.Str> {
    override fun onSuccess(arg: Native.Str) {
        super.onSuccess(arg)
    }

    override fun onErr(code: Int, msg: String) {
        throw Exception("ooops")
    }
}
