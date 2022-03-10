package ch06.ex1_3_1_SafeCallOperator

import java.util.*

fun printAllCaps(s: String?) {
    val allCaps: String? = s?.uppercase(Locale.getDefault())
    println(allCaps)
}

fun main(args: Array<String>) {
    printAllCaps("abc")
    printAllCaps(null)
}
