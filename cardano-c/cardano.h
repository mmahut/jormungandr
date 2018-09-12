#ifndef CARDANO_RUST_H
# define CARDANO_RUST_H
/* Basic Types */

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

typedef int cardano_result;

/*********/
/* BIP39 */
/*********/

cardano_result cardano_bip39_encode(const char * const entropy_raw, unsigned long entropy_size, unsigned short *mnemonic_index, unsigned long mnemonic_size);

/*********/
/* Keys  */
/*********/

typedef struct cardano_xprv cardano_xprv;
typedef struct cardano_xpub cardano_xpub;

cardano_xpub *cardano_xprv_delete(cardano_xprv *privkey);
cardano_xpub *cardano_xprv_to_xpub(cardano_xprv *privkey);

uint8_t *cardano_xprv_to_bytes(cardano_xprv *privkey);
cardano_xprv *cardano_xprv_from_bytes(uint8_t *bytes);

cardano_xpub *cardano_xpub_delete(cardano_xpub *pubkey);

/*************/
/* addresses */
/*************/

typedef struct cardano_address cardano_address;

/* check if an address is a valid protocol address.
 * return 0 on success, !0 on failure. */
int cardano_address_is_valid(const char * address_base58);

cardano_address *cardano_address_new_from_pubkey(cardano_xpub *publickey);
void cardano_address_delete(cardano_address *address);

char *cardano_address_export_base58(cardano_address *address);
cardano_address *cardano_address_import_base58(const char * address_bytes);

/***********/
/* Wallet  */
/***********/

typedef struct cardano_wallet cardano_wallet;
typedef struct cardano_account cardano_account;

cardano_wallet *cardano_wallet_new(const uint8_t * const entropy_ptr, unsigned long entropy_size,
                                   const char * const password_ptr, unsigned long password_size);
void cardano_wallet_delete(cardano_wallet *);

cardano_account *cardano_account_create(cardano_wallet *wallet, const char *alias, unsigned int index);
void cardano_account_delete(cardano_account *account);

unsigned long cardano_account_generate_addresses(cardano_account *account, int internal, unsigned int from_index, unsigned long num_indices, char *addresses_ptr[]);

/****************/
/* Transactions */
/****************/

typedef struct cardano_staging_transaction cardano_staging_transaction;

cardano_staging_transaction *cardano_transaction_new(void);
//cardano_transaction_finalize();
//cardano_transaction_add_input();
//cardano_transaction_add_output();

#ifdef __cplusplus
}
#endif

#endif
