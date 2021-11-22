package com.linkedin.android.rsdroid;

import com.linkedin.android.rpc.NativeImpl
import java.util.*

class RustCore {
    external fun greeting(): String
    external fun run(cmd : Int ,args : ByteArray, cb : ProtoCallback?)
    init {
        System.loadLibrary("rsdroid")
    }

    companion object {
        val instance: RustCore = RustCore()
        val navHelper : NativeHelp = instance.NativeHelp();
    }




    interface Callback<T> {
        fun onSuccess(arg : T)
        fun onErr(code : Int, msg: String)
    }

    interface ProtoCallback {
        fun onSuccess(out : ByteArray) {

        }
        fun onErr(code: Int, msg : String) {

        }
    }

    inner class NativeHelp : NativeImpl() {
        override fun executeCommand(command: Int, args: ByteArray?, cb: ProtoCallback?) {
            run(command, args!!, cb);
        }
    }


}