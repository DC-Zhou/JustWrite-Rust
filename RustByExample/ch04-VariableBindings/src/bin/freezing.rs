fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let mut _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;
        // Comment out this line

        // `_mutable_integer` goes out of scope
    }

    // OK! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}