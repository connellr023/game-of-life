#include "framebuffer.hpp"
#include <windows.h>

extern "C" void create_window(const char *title, uint32_t width, uint32_t height) {
    // Create a window
}

extern "C" void render() {
    // Render the window
}

extern "C" void set_pixel(uint32_t x, uint32_t y, uint32_t color) {
    // Set a pixel
}

extern "C" void destroy_window() {
    // Destroy the window
}