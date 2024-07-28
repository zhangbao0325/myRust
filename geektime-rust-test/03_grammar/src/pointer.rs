use std::fmt;

struct Length(i32);

impl fmt::Pointer for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

fn main() {
    let l = Length(42);
    println!("l is in memory here: {l:p}");

    let l_ptr = format!("{l:018p}");
    assert_eq!(l_ptr.len(), 18);
    assert_eq!(&l_ptr[..2], "0x");
}

/*
这段代码定义了一个名为 Length 的结构体，它包含一个 i32 类型的成员变量。然后，我们为 Length 结构体实现了 fmt::Pointer trait，这个 trait 用于格式化一个指针类型的值。

在 fmt::Pointer trait 的实现中，我们首先获取 self 的指针地址，然后将其传递给 fmt::Pointer::fmt() 方法进行格式化。这样，我们就可以使用 {l:p} 的格式字符串将 Length 类型的值格式化为指针类型的值。

在 main() 函数中，我们创建了一个 Length 类型的实例 l，然后使用 {l:p} 的格式字符串将其格式化为指针类型的值，并打印出来。这样，我们就可以看到 l 实例在内存中的地址。

接下来，我们使用 {l:018p} 的格式字符串将 l 实例的地址格式化为一个固定长度为 18 的字符串，并将其赋值给 l_ptr 变量。然后，我们使用 assert_eq!() 宏检查 l_ptr 的长度是否为 18，并检查字符串的前两个字符是否为 "0x"。

总的来说，这段代码演示了如何在 Rust 中使用 fmt::Pointer trait 将一个自定义类型的值格式化为指针类型的值，并将其打印出来。这可以帮助我们更好地了解 Rust 中的内存模型和指针操作。
*/


/*
self as *const Self;  这是什么用法？ 这个语法好奇怪

`self as *const Self` 是一个类型转换的语法，将 `self` 的引用转换为指向 `Self` 类型对象的常量指针。

在 Rust 中，`*const T` 是一个不可变的原始指针类型，它指向类型 `T` 的内存地址。通过将 `self` 引用转换为 `*const Self`，我们可以获取到 `self` 对象在内存中的地址。

这种类型转换通常用于需要直接操作内存地址的场景，例如在实现 `fmt::Pointer` trait 时，我们需要将一个自定义类型的值格式化为指针类型的值。在这种情况下，我们需要获取该值在内存中的地址，然后将其传递给 `fmt::Pointer::fmt()` 方法进行格式化。

需要注意的是，使用原始指针涉及到底层的内存操作，需要特别小心，以避免出现悬垂指针、空指针解引用等问题。在 Rust 中，原始指针的使用受到严格的限制，需要遵循安全性规则。
*/ 