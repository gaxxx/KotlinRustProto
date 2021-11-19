package com.linkedin.android.kotinrustproto

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.View
import com.linkedin.android.kotinrustproto.databinding.ActivityMainBinding
import com.linkedin.android.rsdroid.RustCore

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
        setContentView(binding.root);
    }
}