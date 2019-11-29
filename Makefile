%: bin/% data/%.in
	$^

bin/%: src/day%.rs
	rustc -o $@ $<
