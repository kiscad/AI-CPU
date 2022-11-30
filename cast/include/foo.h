#ifndef FOO_H
#define FOO_H

#include <memory>

namespace aicpu
{
  uint32_t foo(uint32_t n);

  class CpuKernel
  {
  public:
    uint32_t Compute(uint32_t n);
  };
}

#endif
