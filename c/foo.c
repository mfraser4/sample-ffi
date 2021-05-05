#include <stdlib.h>
#include <stdio.h>
#include "foo.h"

struct Foo *foo_new()
{
	return malloc(sizeof(struct Foo));
}

void foo_free(struct Foo *f)
{
	if (f)
		free(f);
}

void foo_hello_world(struct Foo *f)
{
	if (f)
		printf("hello, world! I have a value of %d\n", f->bar);
	else
		printf("%s: f is not nullable", __func__);
}