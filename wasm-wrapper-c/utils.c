#include "utils.h"

#include "extism/extism-pdk.h"
#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

#define __FILENAME__                                                           \
  (strrchr(__FILE__, '/') ? strrchr(__FILE__, '/') + 1 : __FILE__)

#define PATH_MAX 4096

void print_current_dir() {
  char cwd[PATH_MAX];
  if (getcwd(cwd, sizeof(cwd)) != NULL) {
    char msg3[1024];
    sprintf(msg3, "CWD: %s", cwd);
    extism_log(msg3, strlen(msg3), ExtismLogInfo);

  } else {
    char *msg3 = "could not find cwd";
    extism_log(msg3, strlen(msg3), ExtismLogInfo);
  }
}

void list_current_dir() {
#define _XOPEN_SOURCE 700

  DIR *dp;
  struct dirent *ep;
  dp = opendir("/plugin");
  if (dp != NULL) {
    while ((ep = readdir(dp)) != NULL) {
      char msg3[1024];
      sprintf(msg3, "CWD: %s", ep->d_name);
      extism_log(msg3, strlen(msg3), ExtismLogInfo);
    }

    extism_log("\n", 1, ExtismLogInfo);
    (void)closedir(dp);
  } else {
    extism_log("failed to list", 15, ExtismLogInfo);
    char *logmsg = strerror(errno);
    extism_log(logmsg, strlen(logmsg), ExtismLogInfo);
    perror("utils.c | E | Couldn't open the current directory");
  }
}
