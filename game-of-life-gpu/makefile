# Compiler and flags
CXX = g++
CXXFLAGS = -Wall

# Target executable name
TARGET = build

# Source files
SRCS = \
    src/main.cpp \

# Include path for GLFW
INCLUDES = -IC:/GLFW/include

# Library path and libraries for GLFW
LIBS = -LC:/GLFW/lib-mingw-w64 -lglfw3 -lopengl32 -lgdi32

# Object files
OBJS = $(SRCS:.cpp=.o)

# Default target
all: $(TARGET)

# Rule to build the target executable
$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) $(INCLUDES) -o $@ $^ $(LIBS)

# Rule to build object files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

# Clean up build files
clean:
	del /F /Q $(subst /,\,$(OBJS)) $(TARGET).exe

# Run the application
run: $(TARGET)
	./$(TARGET)

# Release build with optimizations
release: CXXFLAGS += -O2
release: all

# Phony targets
.PHONY: all clean run release
