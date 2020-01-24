CC		:= gcc
CFLAGS	:= -Wall -Wextra -g

# Directories
BINDIR := bin
SRCDIR := src
OBJDIR := obj
IDIR := include
LIBDIR := lib

LIBRARIES :=

ifeq ($(OS),Windows_NT)
EXECUTABLE := main.exe
else
EXECUTABLE := main
endif

SOURCEDIRS := $(shell bash -c find $(SRCDIR) -type d)
INCLUDEDIRS := $(shell bash -c find $(IDIR) -type d)
LIBDIRS := $(shell bash -c find $(LIBDIR) -type d)

CINCLUDES	:= $(patsubst %,-I%, $(INCLUDEDIRS:%/=%))
CLIBS := $(patsubst %,-L%, $(LIBDIRS:%/=%))

# SOURCES		:= $(wildcard $(patsubst %,%/*.c, $(SOURCEDIRS)))
SOURCES := $(wildcard $(SRCDIR)/*.c)
OBJECTS := $(subst $(SRCDIR)/,$(OBJDIR)/,$(SOURCES:.c=.o))


all: $(BINDIR)/$(EXECUTABLE)

.PHONY: clean
clean:
	-$(shell bash -c "$(RM) -f ./$(BINDIR)/$(EXECUTABLE)")
	-$(shell bash -c "$(RM) -f ./$(OBJECTS)")

run: all
	./$(BINDIR)/$(EXECUTABLE)

$(BINDIR)/$(EXECUTABLE): $(OBJECTS)
	$(CC) $(CFLAGS) $(CINCLUDES) $(CLIBS) $^ -o $@ $(LIBRARIES)

$(OBJDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) $(CINCLUDES) $(CLIBS) $^ -o $@ $(LIBRARIES)
