#ifndef FRAMEBUFFER_H
#define FRAMEBUFFER_H
#include <cstdint>
#include <cstdbool>

extern bool is_running;

extern "C" {
    void create_window(const char *title, uint32_t width, uint32_t height);
    void handle_events();
    void render();
    void set_pixel(uint32_t x, uint32_t y, uint32_t color);
    void destroy_window();

    bool is_window_running() {
        return is_running;
    }

    void stop_window() {
        is_running = false;
    }
}

#endif // FRAMEBUFFER_H