#include <iostream>
#include "include/foo.h"

using namespace std;
using namespace aicpu;

int main()
{
  cout << "hello world" << endl;
  foo(32);

  CpuKernel cpu;
  cpu.Compute(123);
  return 0;
}
