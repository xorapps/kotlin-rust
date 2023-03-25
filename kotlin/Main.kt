fun main() {
    System.loadLibrary("rust_kotlin")

    val a: Int = 4
    val b: Int = 2

    val foo = rustyAdd(a, b)

    println("The Rusty Value of Foo is $foo")

    val bar = rustyClass()
    val out = bar.getMessage()

    println(out)

    val bar2 = rustyAcceptReturnClass("YEAH")
    val out2 = bar2.getMessage()

    println(out2)

    val bar3 = rustyArray()

    for (value in bar3) {
        println(value)
    }

    /*val bar4 = rustyHashMap()

    bar4.forEach { entry ->
        val (key, value) = entry
        println("Key: $key, Value: $value")
    }*/
}
