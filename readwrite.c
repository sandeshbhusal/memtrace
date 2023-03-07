#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#define BLOCK_SIZE 4096

int main() {
  int fdr = open("/dev/urandom", O_RDONLY);
  int fdw = open("/tmp/out", O_WRONLY);
  if (fdr == -1 || fdw == -1) {
    perror("Error opening file");
    exit(0);
  }

  char *buffer = malloc(BLOCK_SIZE);
  while (1) {
    // Read BLOCK from fdr and write to fdw.
    int rc = read(fdr, buffer, BLOCK_SIZE);
    if (rc != BLOCK_SIZE) {
      perror("Reading ends here.");
      exit(-1);
    }

    // Write that to the output file.
    int wc = write(fdw, buffer, BLOCK_SIZE);
    if (rc != wc) {
      perror("???");
      exit(-2);
    }

    // Sleep for a second.
    sleep(1);
  }

  free(buffer);
  close(fdr);
  close(fdw);

  return 0;
}
