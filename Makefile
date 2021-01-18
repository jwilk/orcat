# Copyright © 2021 Jakub Wilk <jwilk@jwilk.net>
# SPDX-License-Identifier: MIT

.PHONY: all
all: orcat

orcat: orcat.rs
	rustc -g -O $(<)

.PHONY: clean
clean:
	rm -f orcat

# vim:ts=4 sts=4 sw=4 noet
