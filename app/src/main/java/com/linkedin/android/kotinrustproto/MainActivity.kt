package com.linkedin.android.kotinrustproto

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.View
import com.linkedin.android.kotinrustproto.databinding.ActivityMainBinding
import com.linkedin.android.proto.Native
import com.linkedin.android.rpc.NativeMethods
import com.linkedin.android.rsdroid.RustCore
import com.linkedin.android.rsdroid.RustCore.ProtoCallback
import com.linkedin.android.rsdroid.RustCore.NativeHelp;

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater);
        binding.text.text = RustCore.instance.greeting();

        binding.button.setOnClickListener(View.OnClickListener {

        })
        // call by impl
        RustCore.navHelper.hello(
            Native.HelloIn.newBuilder()
                .setArg(10).build(),
            object : RustCore.Callback<Native.HelloOut> {
            override fun onErr(code: Int, msg: String) {
                Log.d("MainActivity", "msg");
            }

            override fun onSuccess(arg: Native.HelloOut) {
                Log.d("MainActivity", arg.toString());
            }
        });

        // call by method
        RustCore.instance.run(NativeMethods.SINK, Native.Empty.getDefaultInstance().toByteArray(), null);

        setContentView(binding.root);
    }
}