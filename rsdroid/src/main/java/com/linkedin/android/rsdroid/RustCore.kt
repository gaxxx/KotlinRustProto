package com.linkedin.android.rsdroid;

import com.linkedin.android.rpc.NativeImpl
import java.nio.ByteBuffer

class RustCore {
    external fun greeting(): String
    external fun empty()
    external fun emptySet(s : String)
    external fun readString(s : String)

    external fun save(key : String, v : String)
    external fun get(key : String) : String

    external fun emptySetB(s : ByteArray)
    external fun byteArray() : ByteArray
    external fun copyByteArray(b : ByteArray)
    external fun readByteArray(b : ByteArray)
    external fun writeByteArray(b : ByteArray)

    external fun readByteBuffer(b : ByteBuffer)
    external fun writeByteBuffer(b : ByteBuffer)

    external fun run(cmd : Int ,args : ByteArray, cb : ProtoCallback?)
    init {
        System.loadLibrary("rsdroid")
    }

    companion object {
        val instance: RustCore = RustCore()
        val navHelper : NativeHelp = instance.NativeHelp();
    }




    interface Callback<T> {
        fun onSuccess(arg : T) {}
        fun onErr(code : Int, msg: String) {}
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