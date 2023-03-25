/** This file is used as a namespace for all the exported Rust functions. */
@file:JvmName("RustLibrary")

/** Sorts a string's letters in lexicographic order. */
// external fun sortLetters(str: String): String;
external fun rustyAdd(a: Int, b: Int): Int

external fun rustyClass(): RustyClass

external fun rustyAcceptReturnClass(input: String): RustyAcceptReturnClass

external fun rustyArray(): Array<String>

// external fun rustyHashMap(): HashMap<String, String>

// external fun <T> rustyGenericClass(input: T): RustyGenericClass<T>
external fun <T> rustyGenericClass(input: T)
// external fun getString(): String

class RustyClass {
    private val message: String = "UNINIT"

    fun getMessage(): String {
        return message
    }
}

class RustyAcceptReturnClass {
    private val message: String = "UNINIT_RUSTY_ACCEPT_RETURN"

    fun getMessage(): String {
        return message
    }
}
/*
class RustyGenericClass<T>(val input: T) {
    private val message: String = "UNINIT_RUSTY_ACCEPT_RETURN"

    fun getMessage(): String {
        return message
    }
}
*/
