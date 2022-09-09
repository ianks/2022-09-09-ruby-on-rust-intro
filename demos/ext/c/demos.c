#include "demos.h"

VALUE rb_demos(VALUE self, VALUE str)
{
  char *ptr = RSTRING_PTR(str);
  long len = RSTRING_LEN(str);
  char new[len];

  for (int i = 0; i < len; i++)
  {
    new[i] = ptr[len - i - 1];
  }

  return rb_str_new_cstr(new);
}

void Init_c(void)
{
  rb_define_global_function("reverse_it_c", rb_demos, 1);
}
