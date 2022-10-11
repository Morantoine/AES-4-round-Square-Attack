# Crypto Engineering (GBX9SY03)
## TP — Square attack on 31/2 rounds of AES

<p style="text-align:center; font-style:italic; margin-top:-1em; font-weight:500; color:#555/*; font-size: 1.2em*/"> Antoine Moran, Clément Gindrier</p>

### Exercice 1

##### Q.1

`xtime` sert à multiplier par x un polynome de $F_{2⁸}$, c'est-à-dire de $(Z/2Z[X])/m$ avec m un polynome de degré 8 irréductible.

`xtime` prend $m(x) = X⁸ + X⁴ + X³ + X + 1$

L'algorithme décale les bits vers la gauche, ce qui augmente le degré de 1. Si l'on a un overflow. On soustrait $X⁸$, ce qui revient à xorer $0x1B$ (la représentation en binaire de $X⁴ + X³ + X + 1$) car on travaille modulo 2 et modulo m.

Avec $X⁸ + X⁶ + X⁵ + X⁴ + X³ + X + 1$ , il suffit de xorer par $0x7B$  au lieu de $0x1B$.

#### Q.2

```c
void prev_aes128_round_key(const uint8_t next_key[16], uint8_t prev_key[16], int round)
{
	int	i;

	for (i = 15; i > 3; i--)
	{
		prev_key[i] = next_key[i] ^ next_key[i - 4];
	} 
    // a = b ^ c <=> b = a ^ c
	prev_key[0] = next_key[0] ^ S[prev_key[13]] ^ RC[round];
	prev_key[1] = next_key[1] ^ S[prev_key[14]];
	prev_key[2] = next_key[2] ^ S[prev_key[15]];
	prev_key[3] = next_key[3] ^ S[prev_key[12]];
}
```

On teste avec la clé : `000102030405060708090a0b0c0d0e0f`

Ça marche bien.

#### Q.3

- Si l'on prend $k_1 = k_2$, alors  $F(k_1 || k_2, x) = \Epsilon(k_1, x) \oplus \Epsilon(k_2, x) = \Epsilon(k_1, x) \oplus \Epsilon(k_1, x) = 0 $.

- Soit $\lambda$ un $\Lambda$-set, qui vérifie donc le distinguiser pour l'AES. Soient $k1$ et $k2$  deux clefs de 256 bits chacune. On a :

  $\oplus_{x \in \lambda} F(k1 || k2, x) = \oplus_{x \in \lambda} (AES^3(k1, x) \oplus AES^3(k2, x)) = \oplus_{x \in \lambda} AES^3(k1, x) + \oplus_{x \in \lambda} AES^3(k2, x) = 0 \oplus 0 = 0$.

  

  Le distinguisher est donc également valable pour F. 
  
  Le programme de test pour le vérifier

```c
	// the key in the documentation: 000102030405060708090a0b0c0d0e0f
	uint8_t key[16] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};

	// 256 lambda_sets of size 16
	uint8_t lambda_set[256][16] = {0};
	for (uint16_t num_set = 0; num_set < 256; num_set++) {
		lambda_set[num_set][0] = num_set;
	}

	// Encrypt the 256 sets
	for (uint16_t i = 0; i < 256; i++) {
		aes128_enc(lambda_set[i], key, 3, true);
	}

	// Xor the 256 sets in the fisrt set
	for (uint16_t i = 1; i < 256; i++) {
		for (uint9_t j = 0; j < 16; j++) {
			lambda_set[0][j] ^= lambda_set[i][j];
		}
	}

	// print the firt set who have to be full of 0
	printf("[");
	for (uint8_t j = 0; j < 16; j++) {
		printf("%u ", lambda_set[0][j]);
	}
	printf("]\n");
```

Résultat :

```c
[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ]
```



### Exercice 2

#### Q.1.

Aller dans le dossier `attack`, on a tout refait en Rust car c'est plus rigolo. Lancer : `cargo run --release`

#### Q.2.

Oui mais pas tout le temps.
