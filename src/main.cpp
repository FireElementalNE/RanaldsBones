#include <iostream>
#include <unistd.h>
#include <cstdlib>
#include <time.h>
extern void print_usage(char * argv0);
extern int safe_atoi(char * arg);
extern void check_inputs(int t, int g, int b);
extern void roll_dice(int t, int g, int b, bool quiet);
int main(int argc, char ** argv) {
#ifdef _WIN32
	srand (time(NULL));
#endif
	bool quiet = false;
	int tomes = 0, grims = 0, bdice = 0, c;
	while ((c = getopt(argc, argv, "ht:g:b:q")) != -1) {
		switch (c) {
			case 'h':
				print_usage(argv[0]);
				exit(EXIT_SUCCESS);
				break;
			case 't':
				tomes = safe_atoi(optarg);
				break;
			case 'g':
				grims = safe_atoi(optarg);
				break;
			case 'b':
				bdice = safe_atoi(optarg);
				break;
			case 'q':
				quiet = true;
				break;
		}
	}
	check_inputs(tomes, grims, bdice);
	roll_dice(tomes, grims, bdice, quiet);
}