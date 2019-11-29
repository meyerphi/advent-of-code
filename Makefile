%: bin/% data/%.in
	$^

bin/%: src/%.rs
	rustc -o $@ $<
