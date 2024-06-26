A wrapper type  where all standard ops are checked instead.
This means `Checked(a) + Checked(b)` is equivalent to `a.checked_add(&b)`

