// Licensed under the Apache-2.0 license

#ifndef CALIPTRA_API_C_BINDING_H
#define CALIPTRA_API_C_BINDING_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum mailbox_command {
  FIRMWARE_LOAD = 1180126276,
  GET_IDEV_CERT = 1229210947,
  GET_IDEV_INFO = 1229210953,
  POPULATE_IDEV_CERT = 1229210960,
  GET_LDEV_CERT = 1279542614,
  GET_RT_ALIAS_CERT = 1128616530,
  ECDSA384_VERIFY = 1397311318,
  LMS_VERIFY = 1280136022,
  STASH_MEASUREMENT = 1296384339,
  INVOKE_DPE = 1146111299,
  DISABLE_ATTESTATION = 1146307148,
  FW_INFO = 1229866575,
  DPE_TAG_TCI = 1414612820,
  DPE_GET_TAGGED_TCI = 1196705604,
  INCREMENT_PCR_RESET_COUNTER = 1346589266,
  QUOTE_PCRS = 1346589265,
  EXTEND_PCR = 1346589253,
  ADD_SUBJECT_ALT_NAME = 1095521358,
  CERTIFY_KEY_EXTENDED = 1129006424,
  /**
   * The status command.
   */
  VERSION = 1179670098,
  /**
   * The self-test command.
   */
  SELF_TEST_START = 1179667540,
  /**
   * The self-test get results.
   */
  SELF_TEST_GET_RESULTS = 1179667559,
  /**
   * The shutdown command.
   */
  SHUTDOWN = 1179669316,
  /**
   * The capabilities command.
   */
  CAPABILITIES = 1128353875,
  /**
   * The authorization manifest set command.
   */
  SET_AUTH_MANIFEST = 1096043854,
  /**
   * The authorize and stash command.
   */
  AUTHORIZE_AND_STASH = 1096045384,
  /**
   * The get IDevID CSR command.
   */
  GET_IDEV_CSR = 1229210450,
  /**
   * The get FMC Alias CSR command.
   */
  GET_FMC_ALIAS_CSR = 1179468626,
  /**
   * The sign with exported ecdsa command.
   */
  SIGN_WITH_EXPORTED_ECDSA = 1398228293,
  /**
   * The revoke exported CDI handle command.
   */
  REVOKE_EXPORTED_CDI_HANDLE = 1381385032,
} mailbox_command;

typedef MailboxRespHeader mailbox_response_header;

#endif /* CALIPTRA_API_C_BINDING_H */
