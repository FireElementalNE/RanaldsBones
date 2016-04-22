#include <iostream>
#include <cerrno>
#include <cstdio>
#include <cstdlib>
#include <string.h>
#include <inttypes.h>
#ifndef _WIN32
#include <openssl/rand.h>
#endif
using namespace std;
#define MAX_ARG_LEN 256
#define MAX_T 3
#define MAX_G 2
#define MAX_B 2
#define DICE_COUNT 7
#define MAX_ROLL 1000
#define NCUTOFF (1.0 / 3.0) * MAX_ROLL
#define TCUTOFF (2.0 / 3.0) * MAX_ROLL
#define BCUTOFF 499
void print_usage(char * argv0) {
	cout << "usage: " << argv0 << " [-t tomes] [-g grimoires] [-b bonus dice]" << endl;
	cout << "\t-t number of tomes (MAX 3)" << endl;
	cout << "\t-g number of grimoires (MAX 2)" << endl;
	cout << "\t-b number of bonus dice (MAX 2)" << endl;
	exit(EXIT_SUCCESS);
}
int safe_atoi(char * arg) {
	char * tmp;
	if(!(tmp = new char[MAX_ARG_LEN])) {
		perror("new");
		exit(EXIT_FAILURE);
	}
	strncpy(tmp, arg, MAX_ARG_LEN);
	return atoi(tmp);
}
void check_inputs(int t, int g, int b) {
	bool failed = false;
	if(t > MAX_T || t < 0) {
		cerr << "ERROR: Tome count can be one of [0,1,2,3] (Got " << t << ")." << endl;
		failed = true;
	}
	if(g > MAX_G || g < 0) {
		cerr << "ERROR: Grimoire count can be one of [0,1,2] (Got " << g << ")." << endl;
		failed = true;
	}
	if(b > MAX_B || b < 0) {
		cerr << "ERROR: Bonus dice can be one of [0,1,2] (Got " << b << ")." << endl;
		failed = true;
	}
	if(failed) {
		exit(EXIT_FAILURE);
	}
}
#ifdef _WIN32
double get_random_number() {
	int random_num = rand() % MAX_ROLL;
	return (double) random_num;
}
#else
double get_random_number() {
	uint32_t random_num;
	int rc = RAND_bytes((unsigned char *)&random_num, sizeof(uint32_t));
	if(rc == 0) {
		cerr << "crypto: failed." << endl;
		exit(EXIT_FAILURE);
	}
  	if(random_num < 0) {
  		random_num = random_num * -1;
  	}
  	random_num = random_num % MAX_ROLL;
  	return (double)random_num;
}
#endif
int roll(double cutoff) {
	double tmp = get_random_number();
	if(tmp < cutoff) {
		return 1;
	}
	return 0;
}
int roll_dice_helper(int total, string name, int count, double cutoff) {
	for(int i = 0; i < count; i++) {
		int roll_val = roll(cutoff);
		if(roll_val) {
			cout << "> S";
		}
		else {
			cout << "> F";
		}
		cout << " for " << name << " #" << i+1 <<endl; 
		total += roll_val;
	}
	return total;
}
void roll_dice(int t, int g, int b) {
	int total = 0;
	int d = DICE_COUNT - t - g - b; 
	cout << "Dice pool is:" << endl;
	cout << " " << t << " Tome(s)" << endl;
	cout << " " << g << " Grimoire(s)" << endl;
	cout << " " << b << " Bonus Dice" << endl;
	cout << " " << d << " Regular Dice" << endl;
	cout << "-------------------" << endl;
	total = roll_dice_helper(total, "Tome", t, TCUTOFF);
	total = roll_dice_helper(total, "Grimoire", g, MAX_ROLL);
	total = roll_dice_helper(total, "Bonus Dice", b, BCUTOFF);
	total = roll_dice_helper(total, "Regular Dice", d, NCUTOFF);
	cout << "-------------------" << endl;
	cout << "Total is " << total << endl;
}