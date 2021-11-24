package com.linkedin.android.kotinrustproto

import android.content.Context
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.linkedin.android.kotinrustproto.databinding.ActivityMainBinding
import com.linkedin.android.proto.Native
import com.linkedin.android.rsdroid.RustCore

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater);
        binding.text.text = RustCore.instance.greeting();

        val path: String = applicationContext.cacheDir.absolutePath + "/test"
        RustCore.navHelper.open(Native.Str.newBuilder().setVal(path).build(),  object : RustCore.Callback<Native.Resp>{
            override fun onErr(code: Int, msg: String) {
                super.onErr(code, msg)
            }
        });

        val iterCount = 1000;
        binding.button.setOnClickListener({
            var start = System.currentTimeMillis();
            for (i in 0..iterCount) {
                RustCore.navHelper.save(
                    Native.SaveIn.newBuilder()
                        .setKey("test_%d".format(i))
                        .setVal("value_%d_10086".format(i))
                        .build(),
                    object : RustCore.Callback<Native.Resp> {}
                );
            }
            var end = System.currentTimeMillis();

            binding.text.text = "takes %d ms to write".format(end - start);
            for (i in 0..iterCount) {
                RustCore.navHelper.get(
                    Native.Str.newBuilder().setVal("test_%d".format(i)).build(),
                    object : RustCore.Callback<Native.Str> {
                        override fun onSuccess(arg: Native.Str) {
                            if (!arg.getVal().equals("value_%d_10086".format(i))) {
                                // throw RuntimeException("oooops")
                            }
                        }
                    }

                );
            }
            var final = System.currentTimeMillis();
            var append = "\ntakes %d ms to read".format(final - end);
            binding.text.text = binding.text.text.toString() + append
        })


        val sharedPref = getPreferences(Context.MODE_PRIVATE)
        binding.button2.setOnClickListener({
            var start = System.currentTimeMillis();
            for (i in 0..iterCount) {
                sharedPref.edit().putString("test_%d".format(i), "value_%d_10086".format(i)).apply()
            }
            var end = System.currentTimeMillis();

            binding.text.text = "takes %d ms to write".format(end - start);
            for (i in 0..iterCount) {
                val item = sharedPref.getString("test_%d".format(i), "")
                if (!item.equals("value_%d_10086".format(i))) {
                    throw RuntimeException("oooops")
                }
            }
            var final = System.currentTimeMillis();
            var append = "\ntakes %d ms to read".format(final - end);
            binding.text.text = binding.text.text.toString() + append
        })

        setContentView(binding.root);

    }

    fun startMeasure() {
        binding.button2.isEnabled = false
        binding.button.isEnabled = false
    }

    fun endMeasure() {
        binding.button2.isEnabled = true
        binding.button.isEnabled = true
    }
}