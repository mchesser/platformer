all: platformer.exe

platformer.exe: ./src/* ./src/*/*
	rustc -O -L ./libs ./src/main.rs -o platformer.exe

clean:
	rm platformer.exe
