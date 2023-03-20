fun main() {
    System.loadLibrary("rust_kotlin")

    val a: Int = 4;
    val b: Int = 2;

    val foo = rustyAdd(a, b);

    println("The Rusty Value of Foo is $foo");

}
