.PHONY: update-plots update-data

update-plots:
	mkdir -p plots
	cp target/criterion/Throughput/report/lines.svg plots/overview.svg
	cp "target/criterion/Throughput small/report/lines.svg" plots/zoom.svg

update-data:
	cargo bench
