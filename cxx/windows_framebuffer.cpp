#include "framebuffer.hpp"
#include <windows.h>
#include <cassert>

HWND hwnd;
HDC hdc;
HBITMAP hbm;

uint32_t *buffer;

uint32_t fb_width = 0;
uint32_t fb_height = 0;

bool is_running = false;

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
        case WM_CLOSE:
            DestroyWindow(hwnd);
            break;
        case WM_DESTROY:
            PostQuitMessage(0);
            break;
        case WM_PAINT:
            render();
            break;
        default:
            break;
    }

    return DefWindowProcW(hwnd, uMsg, wParam, lParam);
}

extern "C" {
    void create_window(const char *title, uint32_t width, uint32_t height) {
        fb_width = width;
        fb_height = height;

        const wchar_t *wtitle = L"Window";

        WNDCLASSW wc = {0};
        wc.lpfnWndProc = WindowProc;
        wc.hInstance = GetModuleHandle(nullptr);
        wc.lpszClassName = wtitle;
        wc.hCursor = LoadCursor(nullptr, IDC_ARROW);

        if (!RegisterClassW(&wc)) {
            MessageBoxW(nullptr, L"Failed to register window class", L"Error", MB_OK | MB_ICONERROR);
            return;
        }

        RECT rect = { 0, 0, static_cast<LONG>(width), static_cast<LONG>(height) };
        AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);

        hwnd = CreateWindowExW(
            0,
            wtitle,
            wtitle,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            rect.right - rect.left,
            rect.bottom - rect.top,
            nullptr,
            nullptr,
            GetModuleHandle(nullptr),
            nullptr
        );

        if (!hwnd) {
            MessageBoxW(nullptr, L"Failed to create window", L"Error", MB_OK | MB_ICONERROR);
            return;
        }

        hdc = GetDC(hwnd);

        BITMAPINFO bmi = { { 0 } };
        bmi.bmiHeader.biSize = sizeof(bmi.bmiHeader);
        bmi.bmiHeader.biWidth = width;
        bmi.bmiHeader.biHeight = -height;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;

        hbm = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, (void **) &buffer, nullptr, 0);

        if (!hbm) {
            MessageBoxW(nullptr, L"Failed to create DIB section", L"Error", MB_OK | MB_ICONERROR);
            return;
        }

        ShowWindow(hwnd, SW_SHOWDEFAULT);
        UpdateWindow(hwnd);

        is_running = true;
    }

    void handle_events() {
        MSG msg;

        while (PeekMessageW(&msg, nullptr, 0, 0, PM_REMOVE) && is_running) {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    void render() {
        assert(fb_width > 0 && fb_height > 0);

        HDC hdc_mem = CreateCompatibleDC(hdc);
        
        if (!hdc_mem) {
            is_running = false;
            return;
        }

        HBITMAP hbm_old = (HBITMAP) SelectObject(hdc_mem, hbm);
        
        if (!hbm_old) {
            DeleteDC(hdc_mem);

            is_running = false;
            return;
        }

        BitBlt(hdc, 0, 0, fb_width, fb_height, hdc_mem, 0, 0, SRCCOPY);

        SelectObject(hdc_mem, hbm_old);
        DeleteDC(hdc_mem);
    }

    void set_pixel(uint32_t x, uint32_t y, uint32_t color) {
        assert(x < fb_width && y < fb_height);
        buffer[y * fb_width + x] = color;
    }

    void destroy_window() {
        if (hbm) {
            DeleteObject(hbm);
        }

        if (hdc) {
            ReleaseDC(hwnd, hdc);
        }

        if (hwnd) {
            DestroyWindow(hwnd);
        }
    }
}