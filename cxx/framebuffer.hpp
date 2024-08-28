#ifndef FRAMEBUFFER_H
#define FRAMEBUFFER_H
#include <cstdint>

void create_window(const char *title, uint32_t width, uint32_t height);
void render();
void set_pixel(uint32_t x, uint32_t y, uint32_t color);
void destroy_window();

#endif // FRAMEBUFFER_H