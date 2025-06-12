// Licensed under the Apache-2.0 license

#ifndef CALIPTRA_API_C_BINDING_H
#define CALIPTRA_API_C_BINDING_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define Capabilities_SIZE_IN_BYTES 16

#define NUM_PAUSERS 5

typedef struct CommandId CommandId;

typedef struct MailboxReqHeader {
  uint32_t chksum;
} MailboxReqHeader;

typedef struct MailboxRespHeader {
  uint32_t chksum;
  uint32_t fips_status;
} MailboxRespHeader;
#define MailboxRespHeader_FIPS_STATUS_APPROVED 0

typedef struct AddSubjectAltNameReq {
  struct MailboxReqHeader hdr;
  uint32_t dmtf_device_info_size;
  uint8_t dmtf_device_info[MAX_DEVICE_INFO_LEN];
} AddSubjectAltNameReq;
#define AddSubjectAltNameReq_MAX_DEVICE_INFO_LEN 128































































#endif /* CALIPTRA_API_C_BINDING_H */
