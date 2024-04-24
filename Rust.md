# Rust

## Strings

String Literals Are Immutable: In Rust, string literals like "test" are of type &str (a string slice), which is inherently immutable. This means that even though you can change which string slice my_string points to, you cannot change the content of the string slice itself.

Mutable Binding vs Mutable Data: Declaring my_string with mut allows you to rebind my_string to point to a different string slice, but it does not allow you to modify the contents of "test". For example:

```rust
let mut my_string = "test";  // my_string can be rebound, but "test" cannot be altered
my_string = "another test";  // Valid: rebinding to another &str
```

For Mutable String Data: If you need a mutable string that you can change (e.g., append text, modify characters), you should use String instead of &str:

```rust
let mut my_string = String::from("test");  // Now my_string is a mutable String
my_string.push_str("ing");                 // Valid: modifies the String to "testing"
```

## Cross Compilation

Cross-compiling Rust programs for different platforms and architectures from an Apple M1 (ARM architecture) machine involves a few steps. You'll need to install the appropriate target architectures in Rust, set up cross-compilers, and possibly deal with some external dependencies.

### 1. Install Rust

First, ensure you have Rust installed. If not, you can install it using `rustup`, which is the Rust toolchain installer. You can install `rustup` by running the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Add Target Architectures

You'll need to add the target architectures for x86 platforms for Mac, Linux, and Windows. You can add these targets using `rustup`:

- For **macOS** (x86_64):

  ```bash
  rustup target add x86_64-apple-darwin
  ```

- For **Linux** (x86_64):

  ```bash
  rustup target add x86_64-unknown-linux-gnu
  ```

- For **Windows** (x86_64):

  ```bash
  rustup target add x86_64-pc-windows-gnu
  ```

### 3. Install Cross-Compilation Tools

#### macOS

For macOS, you generally won't need additional tools beyond Rust itself when compiling for another macOS architecture.

#### Linux and Windows

For Linux and Windows, cross-compilation can be more involved due to the need for appropriate linkers and system libraries.

- **Linux:** You might need to install `gcc` for Linux targeting. However, on M1 Macs, you'll typically use a cross-compilation tool like `osxcross`. You can set this up by cloning the `osxcross` repository and following the build instructions.
- **Windows:** For Windows, you can use tools like `mingw-w64` to provide the Windows cross-compiler. Install it via Homebrew:

  ```bash
  brew install mingw-w64
  ```

### 4. Configure Cargo for Cross-Compiling

You'll need to tell Cargo how to find and use the cross-compilers. This is done through a `.cargo/config.toml` file in your project directory or your home directory.

Here’s an example configuration for Windows:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

Replace the linker path with the actual path to your cross-compiler if it's different.

### 5. Build Your Project

Once everything is set up, you can build your project for the specific target using `cargo build`:

- For **macOS**:

  ```bash
  cargo build --target x86_64-apple-darwin
  ```

- For **Linux**:

  ```bash
  export CC=x86_64-linux-musl-gcc
  RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --target x86_64-unknown-linux-musl
  ```

- For **Windows**:

  ```bash
  cargo build --target x86_64-pc-windows-gnu
  ```

### Additional Notes

- **Dependencies:** If your project depends on native libraries, you may need to ensure these libraries are also available for the target platform in a form that the cross-compiler can link against.
- **Testing:** It's important to test your binaries on the target platform to ensure they work as expected. Differences in the operating system APIs, file systems, and other environmental factors can affect behavior.

Cross-compiling can be complex due to these dependencies and configurations, so it may require some troubleshooting to get everything working smoothly.

## Ownership

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are.

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
```

You’ll get an error because Rust prevents you from using the invalidated reference.

### Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? Refernces.

### Refernces

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The scope in which the variable s is valid is the same as any function parameter’s scope, but the value pointed to by the reference is not dropped when s stops being used, because s doesn’t have ownership.
We call the action of creating a reference borrowing.

This will not compile. Because it tries to return a reference to a String that is deallocated when the dangle function finishes executing. This is a fundamental violation of Rust's ownership rules, specifically related to its lifetime handling.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Possible fixes:

- Change the function to return the String directly, not a reference. This transfers ownership out of the function, ensuring the String's data remains valid.

- If you must return a reference for some reason, you'll need to ensure that the String lives longer than the function call. This typically involves changing your approach, perhaps using lifetimes in a different way or restructuring your code to avoid the need for returning a reference to a locally created String.

```rust
fn main() {
    let string = dangle(); // `string` now takes ownership of the `String` returned by `dangle`
}

fn dangle() -> String {
    let s = String::from("hello");
    s // return `s` directly
}
```

The scope in which the variable s is valid is the same as any function parameter’s scope, but the value pointed to by the reference is not dropped when s stops being used, because s doesn’t have ownership.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First we change s to be mut. Then we create a mutable reference with &mut s where we call the change function, and update the function signature to accept a mutable reference with some_string: &mut String. This makes it very clear that the change function will mutate the value it borrows.

**Mutable references have one big restriction:**
If we have an immutable reference to something, we cannot also take a mutable reference.

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
We also cannot have a mutable reference while we have an immutable one to the same value.

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

### The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

## Structs

### Creating Instances from Other Instances with Struct Update Syntax

The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance. Below is an example where all the values stay the same, except the email.

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

- Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section. In this example, we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

### Ownership of Struct Data

In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

The compiler will complain that it needs lifetime specifiers.

### Method Syntax

We’ve put all the things we can do with an instance of a type in one `impl` block.
All functions defined within an impl block are called associated functions because they’re associated with the type named after the `impl`.

Each struct is allowed to have multiple `impl` blocks.

Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.

```rust
impl Rectangle {
    fn square(size: u32) -> Self {      //The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
        Self {
            width: size,
            height: size,
        }
    }
}
```

To call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);`.

We chose `&self` here for the same reason we used `&Rectangle` in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

Enums with structs:

```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```

We can put data directly into each enum variant. We attach data to each variant of the enum directly, so there is no need for an extra struct:

```rust
   enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

`IpAddr::V4()` is a function call that takes a String argument and returns an instance of the `IpAddr` type. We automatically get this constructor function defined as a result of defining the enum.

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

### The Match Control Flow

It can evaluate any type, unlike `if` that can handle only booleans.
The arms’ patterns must cover all possibilities.

Below there are 4 arms: An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value `Coin::Penny` and then the `=>` operator that separates the pattern and the code to run.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
And there is a catch-all:

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```

Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: `_` is a special pattern that matches any value and does not bind to that value.

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

#### Control Flow With If Let

Option<u8> value in the config_max variable but only wants to execute code if the value is the Some variant.

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
```

Instead, we could write this in a shorter way using `if let`:

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

The pattern is `Some(max)`, and the `max` binds to the value inside the `Some`. We can then use `max` in the body of the if let block in the same way we used max in the corresponding `match` arm. The code in the if let block isn’t run if the value doesn’t match the pattern.


## Cross Compilation

I  made a script to compile for Windows, Linux and MacOS on ARM and x86.

```bash
#! /opt/homebrew/bin/bash
# Define target architectures and their corresponding linkers
declare -A targets=(
    [x86_64-unknown-linux-musl]="x86_64-unknown-linux-gnu-gcc" #works
    [arm-unknown-linux-gnueabihf]="arm-linux-gnueabihf-gcc"
    [x86_64-pc-windows-gnu]="x86_64-w64-mingw32-gcc" #works
    [i686-pc-windows-msvc]="i686-w64-mingw32-gcc"
    [x86_64-apple-darwin]="clang" #works
    [aarch64-apple-darwin]="clang" #works
)

# Path to the Cargo config file
CARGO_CONFIG_PATH=".cargo/config.toml"

# Ensure the Cargo config directory exists
mkdir -p $(dirname "$CARGO_CONFIG_PATH")

# Start building the Cargo config
echo "# This file is auto-generated by cross_compile.sh" > "$CARGO_CONFIG_PATH"

# Append config for each target
for target in "${!targets[@]}"; do
    linker="${targets[$target]}"
    echo "[target.${target}]" >> "$CARGO_CONFIG_PATH"
    echo "linker = \"${linker}\"" >> "$CARGO_CONFIG_PATH"
done

# echo "Checking and installing necessary cross-compilers and tools..."
# Adjust the installation commands according to your system's package manager or requirements
# macOS specific installations
# brew install mingw-w64 cmake

# Build for each target
for target in "${!targets[@]}"; do
    echo "Building for target $target..."
    cargo build --target "$target" || echo "Failed to build for target $target"
done

echo "Cross-compilation completed."
```

For linux on ARM I had to use Docker:

```Dockerfile
    # Use a Linux base image with Rust installed
    FROM rust:latest

    # Install dependencies needed for cross-compiling and other tools
    RUN apt update && apt upgrade -y && \
        apt install -y clang lld make

    # Install the Rust target for Windows ARM64
    RUN rustup target add aarch64-pc-windows-msvc

    # Set the linker to lld-link (from LLVM), which can handle Windows binaries
    ENV CARGO_TARGET_AARCH64_PC_WINDOWS_MSVC_LINKER=lld-link

    # Set up the working directory
    WORKDIR /app

    # Default command to compile the project for Windows ARM64
    CMD ["cargo", "build", "--target", "aarch64-pc-windows-msvc"]
```

For Windows on x86 I couldn't figure it out so I am living it out for now.
