#include <memory>
#include <iostream>
#include "../include/foo.h"

namespace aicpu
{
  uint32_t foo(uint32_t n)
  {
    std::cout << "hello from foo " << n << std::endl;
    return 0;
  }

  uint32_t CpuKernel::Compute(uint32_t n)
  {
    std::cout << "CpuKernel Compute" << n << std::endl;
    return 0;
  }
}
