number x = 4 + 5;
number y = x + x;

x = 2;

if x == 1 + 1 {
    x = 1;
    print x;
    print x + 9 - 3 - 2 + x;
    print false;
}

if false == false {
    x = 0;
    print x;

    if true {
        bool x = true;
        print x;
    }
}

print x;

# Prints 3
print 5 - 4 + 1 - 1 - 1 + 3;
# Prints 6
print 4 / x + x * 5 - 1 * x * 3;
