docker:
	build:
		docker build --tag newsletter --file Dockerfile .

	run:
		docker run --rm --name newsletter -p 8000:8000 newsletter