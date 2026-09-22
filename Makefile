dev:
		cargo watch -x run

run:
		cargo run

health:
		curl http://192.168.1.157:3000/health-check

