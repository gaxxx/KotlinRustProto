package com.linkedin.android.rsdroid;

class RustCore {

    external fun greeting(): String
    external fun callback(cb : Callback)
    external fun run(cmd : Int ,args : ByteArray, cb : ProtoCallback)
    init {
        System.loadLibrary("rsdroid")
    }

    companion object {
        val instance: RustCore = RustCore()
    }


    interface Callback {
        fun onSuccess()
    }

    interface ProtoCallback {
        fun onSuccess(out : ByteArray) {

        }
        fun onErr(code: Int, msg : String) {

        }
    }
}