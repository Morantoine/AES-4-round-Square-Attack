#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include "aes-128_enc.h"

int main() {
	// the key in the documentation: 000102030405060708090a0b0c0d0e0f
	uint8_t key[16] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};

	// 256 lambda_sets of size 16
	uint8_t lambda_set[256][16] = {0};
	for (uint16_t num_set = 0; num_set < 256; num_set++) {
		lambda_set[num_set][0] = num_set;
	}

	// Encrypt the 256 sets
	for (uint16_t i = 0; i < 256; i++) {
		aes128_enc(lambda_set[i], key, 4, false);
	}
	printf("[");
	for (uint16_t j = 0; j < 255; j++) {
		printf("%u ", lambda_set[j][0]);
	}
	printf("]\n");
/*
	// Xor the 256 sets in the fisrt set
	for (uint16_t i = 1; i < 256; i++) {
		for (uint16_t j = 0; j < 16; j++) {
			lambda_set[0][j] ^= lambda_set[i][j];
		}
	}
*/
	// print the firt set who have to be full of 0
	printf("[");
	for (uint16_t j = 0; j < 16; j++) {
		printf("%u ", lambda_set[0][j]);
	}
	printf("]\n");
	return 0;
}
