# vim: set ft=make ts=2 sw=2 noet:

alias r := run

run file:
	cargo run --release -- {{file}}

clean:
	rm -rf output output.ll
