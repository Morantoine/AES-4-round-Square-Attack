#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/types.h>
#include "aes-128_enc.c"

int main (int argc, char *argv[])
{
	uint8_t ekey[16] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
	uint8_t res[16];
	uint8_t prev_res[16];

	next_aes128_round_key(ekey, res, 1);
	prev_aes128_round_key(res, prev_res, 1);
	for (int i = 0; i < 16; i++) {
		printf("%d, ", ekey[i]);
	}
	printf("\n");
	for (int i = 0; i < 16; i++) {
		printf("%d, ", res[i]);
	}
	printf("\n");
	for (int i = 0; i < 16; i++) {
		printf("%d, ", prev_res[i]);
	}

	return 0;
}
