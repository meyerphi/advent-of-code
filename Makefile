%: src/day%.rs data/%.in
	cargo run --bin=day$@ data/$@.in
