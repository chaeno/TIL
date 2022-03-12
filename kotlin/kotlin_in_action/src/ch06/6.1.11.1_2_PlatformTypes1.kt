package ch06.ex1_11_1_2_PlatformTypes1

import java.util.*

class Person(val name: String?)

fun yellAtSafe(person: Person) {
    println((person.name ?: "Anyone").uppercase(Locale.getDefault()) + "!!!")
}

fun main(args: Array<String>) {
    yellAtSafe(Person(null))
}