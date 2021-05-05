CC=gcc
FLAGS=-std=gnu99
CFLAGS=-Wall -Wextra -fPIC
LDFLAGS=-shared

DESTDIR=$(shell echo $(HOME))
PREFIX=$(DESTDIR)/opt
INCLUDEDIR=$(PREFIX)/include
LIBDIR=$(PREFIX)/lib

TARGET=libfoo.so
SOURCES=$(wildcard *.c)
HEADERS=$(wildcard *.h)
OBJECTS=$(SOURCES:.c=.o)

.PHONY: all
all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CC) $(FLAGS) $(LDFLAGS) $(CFLAGS) $(OBJECTS) -o $@

.PHONY: install
install: all
	mkdir -p $(PREFIX)/lib
	mkdir -p $(PREFIX)/include
	cp $(TARGET) $(LIBDIR)
	cp $(HEADERS) $(INCLUDEDIR)

.PHONY: clean
clean:
	rm -f $(OBJECTS) $(TARGET)

.PHONY: uninstall
uninstall:
	for f in $(HEADERS); do rm -f $(INCLUDEDIR)/$$f; done
	rm -f $(LIBDIR)/$(TARGET)
