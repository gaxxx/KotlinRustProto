package com.linkedin.android.rsdroid;

class RustCore {

    external fun greeting(): String
    external fun callback(cb : Callback)
    init {
        System.loadLibrary("rsdroid")
    }

    companion object {
        val instance: RustCore = RustCore()
    }


    interface Callback {
        fun onSuccess()
    }

}