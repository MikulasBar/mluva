number x = 4 + 5;
number y = x + x;

x = 6;

print x;
print y;

bool a = true;
bool b = false;

print a;
print b;

###########################
x = 2;

if x == 2 {
    x = 1;
    print x;
    print 2 + 9 + 3;
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