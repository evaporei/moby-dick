#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <sched.h>
#include <sys/wait.h>
#include <errno.h>

#define STACKSIZE (1024*1024)
static char child_stack[STACKSIZE];

struct clone_args {
  char **argv;
};

// this will be executed as the result of clone
static int child_exec(void *stuff)
{
  struct clone_args *args = (struct clone_args *)stuff;

  if (execvp(args->argv[0], args->argv) != 0) {
    fprintf(stderr, "failed to execvp arguments %s\n", strerror(errno));
    exit(-1);
  }

  // we should never reach this code
  exit(EXIT_FAILURE);
}

int main(int argc, char **argv)
{
  struct clone_args args;
  args.argv = &argv[1];

  int clone_flags = SIGCHLD;

  // this is the pid of the new process cloned
  pid_t pid = clone(child_exec, child_stack + STACKSIZE, clone_flags, &args);

  if (pid < 0) {
    fprintf(stderr, "clone failed %s\n", strerror(errno));
    exit(EXIT_FAILURE);
  }

  // wait child process to finish, so that parent doesn't exit first
  if (waitpid(pid, NULL, 0) == -1) {
    fprintf(stderr, "failed to wait child process of pid %d\n", pid);
    exit(EXIT_FAILURE);
  }

  exit(EXIT_SUCCESS);
}
