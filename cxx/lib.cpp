#include "lib.hpp"
#include <iostream>

extern "C" void say_hello() {
    std::cout << "Hello, World!" << std::endl;
}