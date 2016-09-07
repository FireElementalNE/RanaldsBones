CC=g++
CFLAGS=-g -Wall
ifneq ($(OS),Windows_NT)
	CFLAGS += -std=c++11 -lssl -lcrypto
endif
OBJ=functions.o
all: main
main: main.cpp $(OBJ)
	$(CC) -o main main.cpp $(OBJ) $(CFLAGS)
alphabet.o: functions.cpp
	$(CC) -c -o functions.o functions.cpp $(CFLAGS)
clean:
ifneq ($(OS),Windows_NT)
	rm -f main *.o
else
	rm main.exe *.o
endif

	