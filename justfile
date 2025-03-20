test-show:
    source .env && cargo run -- show -p=-1 -s -f "dags" -o "json"
