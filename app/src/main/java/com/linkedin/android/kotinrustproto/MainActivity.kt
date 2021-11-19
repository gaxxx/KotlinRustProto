package com.linkedin.android.kotinrustproto

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.View
import com.linkedin.android.kotinrustproto.databinding.ActivityMainBinding
import com.linkedin.android.proto.AdBackend
import com.linkedin.android.rsdroid.RustCore
import com.linkedin.android.rsdroid.RustCore.ProtoCallback

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater);
        binding.text.text = RustCore.instance.greeting();

        binding.button.setOnClickListener(View.OnClickListener {
            RustCore.instance.callback(object : RustCore.Callback {
                override fun onSuccess() {
                    binding.text.text = "Changed";
                }
            });
        })
        val builder = AdBackend.HelloIn.newBuilder();
        val arg = builder.setArg(1000).build();
        RustCore.instance.run(1, arg.toByteArray(), object : ProtoCallback {
            override fun onErr(code: Int, msg: String) {
                Log.d("MainActivity", "msg");
            }

            override fun onSuccess(out: ByteArray) {
                val helloOut = AdBackend.HelloOut.parseFrom(out);
                Log.d("MainActivity", helloOut.toString());
            }
        });
        setContentView(binding.root);
    }
}