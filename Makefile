# Copyright © 2021 Jakub Wilk <jwilk@jwilk.net>
# SPDX-License-Identifier: MIT

PREFIX = /usr/local
DESTDIR =

bindir = $(PREFIX)/bin

.PHONY: all
all: orcat

.PHONY: install
install: orcat
	install -d $(DESTDIR)$(bindir)
	install -m755 $(<) $(DESTDIR)$(bindir)/

orcat: orcat.rs
	rustc -g -O $(<)

.PHONY: clean
clean:
	rm -f orcat

# vim:ts=4 sts=4 sw=4 noet
